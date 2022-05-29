use crate::prelude::*;
use std::time::Duration;

const FIXED_STAGE_MS: u64 = 16;

pub struct ProjectilePlugin;

use crate::td_mode::enemy::CenterOfMass;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        let mut early_fixed_stage = SystemStage::parallel();
        let mut fixed_stage = SystemStage::parallel();
        let mut late_fixed_stage = SystemStage::parallel();
        let mut last_fixed_stage = SystemStage::parallel();
        let mut final_fixed_stage = SystemStage::parallel();
        early_fixed_stage.add_system(aim_projectiles);
        fixed_stage
            .add_system(move_projectiles)
            .add_system(handle_fire_delay);
        late_fixed_stage.add_system(projectile_enemy_collisions);
        late_fixed_stage.add_system(projectile_terrain_collisions);
        last_fixed_stage.add_system(progress_projectile_lifespans);
        final_fixed_stage.add_system(handle_projectile_collisions);

        app.add_startup_system(initialize_projectile_models)
            .add_system_to_stage(CoreStage::PreUpdate, handle_projectile_spawn_messages)
            .add_system(set_initial_direction)
            .add_system(set_initial_speed)
            .add_system(position_delayed_projectiles)
            .add_system(set_initial_damage)
            .add_system_to_stage(CoreStage::PostUpdate, apply_spread)
            .add_system_to_stage(CoreStage::PostUpdate, spread_projectiles)
            .add_system(add_homing)
            .add_system(add_explosive)
            .add_stage_before(
                CoreStage::Update,
                "projectile_fixed_stage",
                FixedTimestepStage::new(std::time::Duration::from_millis(FIXED_STAGE_MS))
                    .with_stage(early_fixed_stage)
                    .with_stage(fixed_stage)
                    .with_stage(late_fixed_stage)
                    .with_stage(last_fixed_stage)
                    .with_stage(final_fixed_stage),
            );
    }
}

struct ProjectileModels {
    ballista_bolt: Handle<Scene>,
}

pub enum ProjectileType {
    Ballista,
}

#[derive(Component)]
pub struct SpawnProjectile(ProjectileType);

#[derive(Component)]
pub struct Projectile(ProjectileType);

#[derive(Component)]
struct AlreadySpawned(Vec<Entity>);

#[derive(Component)]
struct Lifetime(Duration);

#[derive(Component)]
struct ProjectileCollision;

fn initialize_projectile_models(assets: Res<AssetServer>, mut commands: Commands) {
    let projectile_models = ProjectileModels {
        ballista_bolt: assets.load("models/towers/weapons/ballista_projectile.glb#Scene0"),
    };

    commands.insert_resource(projectile_models);
}

pub fn spawn_projectile_message(
    src: Entity,
    target: Entity,
    projectile_type: ProjectileType,
    commands: &mut Commands,
) {
    commands
        .spawn()
        .insert(Message)
        .insert(Sender(src))
        .insert(Target(target))
        .insert(SpawnProjectile(ProjectileType::Ballista));
}

fn spawn_ballista_bolt(
    start_position: Vec3,
    spawn_spread: Vec3,
    models: &Res<ProjectileModels>,
    commands: &mut Commands,
    parent: Entity,
    target: Entity,
) -> Entity {
    let random_fire_delay = thread_rng().gen_range(500..1000);
    commands
        .spawn()
        .insert(Speed(0.1))
        .insert(Projectile(ProjectileType::Ballista))
        .insert(Lifetime(Duration::from_secs(4)))
        .insert(FireDelay(Timer::new(
            Duration::from_millis(random_fire_delay),
            false,
        )))
        .insert(SpawnPosition(start_position))
        .insert(SpreadPosition(start_position + spawn_spread))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(start_position),
        ))
        .insert(Parent(parent))
        .insert(Target(target))
        .with_children(|p| {
            p.spawn_scene(models.ballista_bolt.clone());
        })
        .id()
}

fn handle_projectile_spawn_messages(
    message_query: Query<(Entity, &Message, &Sender, &Target, &SpawnProjectile)>,
    spawn_point_query: Query<(&Transform, &ProjectileSpawnPoint)>,
    enemy_query: Query<&Enemy>,
    multishot_query: Query<&Multishot>,
    models: Res<ProjectileModels>,
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(entity, _, sender, target, _)| {
            if let Ok((spawn_tform, spawn_point)) = spawn_point_query.get(sender.0) {
                if let Ok(_) = enemy_query.get(target.0) {
                    let mut projectile_count = 1;
                    if let Ok(multishot) = multishot_query.get(sender.0) {
                        projectile_count = multishot.0;
                    }

                    const SPAWN_POINT_SPREAD: f32 = 0.55;
                    let projectiles = (0..projectile_count)
                        .map(|_| {
                            let spawn_spread = Vec3::new(
                                thread_rng().gen_range(-SPAWN_POINT_SPREAD..SPAWN_POINT_SPREAD),
                                thread_rng().gen_range(0.0..SPAWN_POINT_SPREAD * 2.0),
                                thread_rng().gen_range(-SPAWN_POINT_SPREAD..SPAWN_POINT_SPREAD),
                            );
                            spawn_ballista_bolt(
                                spawn_tform.translation + spawn_point.0,
                                spawn_spread,
                                &models,
                                &mut commands,
                                current_map.0.unwrap(),
                                target.0,
                            )
                        })
                        .collect();
                    commands.entity(entity).insert(AlreadySpawned(projectiles));
                }
            } else {
                error!(
                "Attempted to spawn a projectile from a source with no transform or spawn point."
            )
            }
            commands.entity(entity).insert(IsHandled);
        });
}

fn add_homing(
    homing_query: Query<&Homing>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .filter(|(_, _, sender, _, _)| homing_query.get(sender.0).is_ok())
        .for_each(|(_, _, _, target, spawned)| {
            spawned.0.iter().for_each(|e| {
                commands.entity(*e).insert(Homing).insert(Target(target.0));
            });
        });
}

fn add_explosive(
    explosive_query: Query<&Explosive>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .filter(|(_, _, sender, _, _)| explosive_query.contains(sender.0))
        .for_each(|(_, _, sender, target, spawned)| {
            spawned.0.iter().for_each(|e| {
                let explosive = explosive_query.get(sender.0).unwrap();
                commands
                    .entity(*e)
                    .insert(*explosive)
                    .insert(Target(target.0));
            });
        });
}

fn set_initial_damage(
    damage_query: Query<&Damage>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .filter(|(_, _, sender, _, _)| damage_query.get(sender.0).is_ok())
        .for_each(|(_, _, sender, _, spawned)| {
            spawned.0.iter().for_each(|e| {
                commands
                    .entity(*e)
                    .insert(damage_query.get(sender.0).unwrap().clone());
            });
        });
}

fn set_initial_speed(
    speed_query: Query<&Speed>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .filter(|(_, _, sender, _, _)| speed_query.get(sender.0).is_ok())
        .for_each(|(_, _, sender, _, spawned)| {
            spawned.0.iter().for_each(|e| {
                commands
                    .entity(*e)
                    .insert(speed_query.get(sender.0).unwrap().clone());
            });
        });
}

fn set_initial_direction(
    transform_query: Query<&Transform>,
    enemy_query: Query<(&Transform, &CenterOfMass)>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .for_each(|(_, _, _, target, spawned)| {
            spawned.0.iter().for_each(|e| {
                let proj_transform = transform_query.get(*e);
                let target_transform = enemy_query.get(target.0);

                let new_transform = proj_transform.unwrap().looking_at(
                    target_transform.unwrap().0.translation + target_transform.unwrap().1 .0,
                    Vec3::Y,
                );
                commands.entity(*e).insert(new_transform);
            });
        });
}

fn position_delayed_projectiles(
    delayed_proj_query: Query<(
        Entity,
        &SpawnPosition,
        &SpreadPosition,
        &Transform,
        &FireDelay,
    )>,
    mut commands: Commands,
) {
    delayed_proj_query
        .iter()
        .for_each(|(entity, spawn, spread, tform, fire_delay)| {
            let pct = ((fire_delay.0.elapsed().as_millis() + 100) as f32
                / fire_delay.0.duration().as_millis() as f32)
                .clamp(0.0, 1.0);
            let scale_mult = 0.5 + (0.5 * pct);
            let new_pos = spawn.0.lerp(spread.0, pct);
            let new_scale = Vec3::ONE * scale_mult;
            commands
                .entity(entity)
                .insert(tform.with_translation(new_pos).with_scale(new_scale));
        });
}

fn aim_projectiles(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Target)>,
    transform_query: Query<(&Transform, &CenterOfMass)>,
    homing_query: Query<&Homing>,
    delay_query: Query<&FireDelay>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(entity, p_transform, _, target)| {
            if let Ok((target_transform, target_center_of_mass)) = transform_query.get(target.0) {
                let target_pos = target_transform.translation + target_center_of_mass.0;
                let new_transform = p_transform.looking_at(target_pos, Vec3::Y);

                commands.entity(entity).insert(new_transform);

                if !homing_query.contains(entity) && !delay_query.contains(entity) {
                    commands.entity(entity).remove::<Target>();
                }
            }
        });
}

fn move_projectiles(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Speed), Without<FireDelay>>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(entity, p_transform, _, speed)| {
            let new_transform = p_transform;
            let new_transform = new_transform
                .with_translation(new_transform.translation + new_transform.forward() * speed.0);

            commands.entity(entity).insert(new_transform);

            if new_transform.translation.y <= 0.0 {
                /*
                commands
                    .spawn()
                    .insert(Message)
                    .insert(Target(entity))
                    .insert(ProjectileCollision);
                */
            }
            /*
                let remaining_dist = new_transform.translation.distance(target_pos);
                if remaining_dist < 0.25 {
                    commands.entity(entity).despawn_recursive();

                    // TODO: Make this into a message instead.
                    commands.entity(target.0).despawn_recursive();

                    let gold_gained = thread_rng().gen_range(5..15);
                    crate::td_mode::gold::send_change_gold_message(&mut commands, gold_gained);
                } else {
                    // Move towards the projectile's target.
                }
            } else {
                commands.entity(entity).despawn_recursive();
            }
                */
        });
}

fn progress_projectile_lifespans(
    projectile_query: Query<(Entity, &Projectile, &Lifetime)>,
    mut commands: Commands,
) {
    projectile_query.iter().for_each(|(e, _, lifetime)| {
        let new_duration = lifetime
            .0
            .saturating_sub(Duration::from_millis(FIXED_STAGE_MS));

        if new_duration == Duration::ZERO {
            commands.entity(e).despawn_recursive();
        } else {
            commands.entity(e).insert(Lifetime(new_duration));
        }
    });
}

fn projectile_terrain_collisions(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Damage)>,
    map_query: Query<&TileMap>,
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    projectile_query.iter().for_each(|(proj_e, proj_t, _, _)| {
        if proj_t.translation.y < 0.1 {
            if let Ok(map) = map_query.get(current_map.0.unwrap()) {
                let coord = map.get_coord_at_pos(proj_t.translation.x, proj_t.translation.z);
                trace!("Projectile collided with tile at coord: {coord:?}");

                if let Ok(tile) = map.get_tile_entity_at_coord(coord) {
                    commands
                        .spawn()
                        .insert(Message)
                        .insert(ProjectileCollision)
                        .insert(Sender(proj_e))
                        .insert(Target(tile));
                }
            }
        }
    });
}

fn projectile_enemy_collisions(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Damage)>,
    enemy_query: Query<(Entity, &Transform, &CenterOfMass, &Enemy)>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(proj_e, proj_t, _, damage)| {
            enemy_query
                .iter()
                .for_each(|(enemy_e, enemy_t, enemy_center, _)| {
                    if proj_t
                        .translation
                        .distance(enemy_t.translation + enemy_center.0)
                        < 0.25
                    {
                        trace!("Dealing {} damage to an enemy.", damage.0);

                        commands
                            .spawn()
                            .insert(Message)
                            .insert(ProjectileCollision)
                            .insert(Sender(proj_e))
                            .insert(Target(enemy_e));

                        commands
                            .spawn()
                            .insert(Message)
                            .insert(Harm(damage.0))
                            .insert(Target(enemy_e));
                    }
                });
        });
}

fn apply_spread(
    transform_query: Query<&Transform>,
    spread_query: Query<&Spread>,
    projectile_messages: Query<(
        &Message,
        &SpawnProjectile,
        &Sender,
        &Target,
        &AlreadySpawned,
    )>,
    mut commands: Commands,
) {
    projectile_messages
        .iter()
        .filter(|(_, _, sender, _, _)| spread_query.get(sender.0).is_ok())
        .for_each(|(_, _, sender, target, spawned)| {
            let spread = spread_query.get(sender.0).unwrap();
            spawned.0.iter().for_each(|e| {
                commands.entity(*e).insert(spread.clone());
            });
        });
}

fn handle_projectile_collisions(
    message_query: Query<(Entity, &Sender, &Target), (With<ProjectileCollision>, With<Message>)>,
    transform_query: Query<&Transform>,
    center_query: Query<&CenterOfMass>,
    mut commands: Commands,
) {
    message_query.iter().for_each(|(entity, proj, other)| {
        let proj_t = transform_query
            .get(proj.0)
            .expect("Failed to get Projectile Transform");
        let pin_to_transform = transform_query
            .get(other.0)
            .expect("Failed to get pinned to transform");
        let mut center = Vec3::ZERO + proj_t.back() * 0.1;

        if let Ok(enemy_center) = center_query.get(other.0) {
            center += enemy_center.0;
        }
        let mut offset = proj_t.with_translation(center);

        let y_rot_of_target = pin_to_transform.rotation.to_euler(EulerRot::ZXY).2;
        offset.rotate(Quat::from_rotation_y(-y_rot_of_target));

        trace!(
            "Handling Collision for Projectile {:?} with Target {:?}",
            proj.0,
            other.0
        );
        commands
            .entity(proj.0)
            .remove::<Damage>()
            .remove::<Speed>()
            .insert(Lifetime(Duration::from_secs(thread_rng().gen_range(2..=5))))
            .insert(offset)
            .insert(Parent(other.0));

        commands.entity(entity).insert(IsHandled);
    });
}

fn handle_fire_delay(mut delay_query: Query<(Entity, &mut FireDelay)>, mut commands: Commands) {
    delay_query.iter_mut().for_each(|(e, mut delay)| {
        delay.0.tick(Duration::from_millis(FIXED_STAGE_MS));
        if delay.0.finished() {
            commands.entity(e).remove::<FireDelay>();
        }
    });
}

fn spread_projectiles(
    proj_query: Query<(Entity, &Transform, &Spread), (With<Projectile>, Without<FireDelay>)>,
    mut commands: Commands,
) {
    proj_query.iter().for_each(|(e, proj_transform, spread)| {
        let forward_point = proj_transform.translation + proj_transform.forward();
        let offset = proj_transform.right() * thread_rng().gen_range(-spread.0..spread.0)
            + proj_transform.up() * thread_rng().gen_range(-spread.0..spread.0);

        commands
            .entity(e)
            .insert(proj_transform.looking_at(forward_point + offset, Vec3::Y))
            .remove::<Spread>();
    });
}
