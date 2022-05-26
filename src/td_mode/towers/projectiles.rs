use crate::prelude::*;
use std::time::Duration;

const PROJECTILE_PROPERTIES_LABEL: &str = "apply_projectile_properties";
const FIXED_STAGE_MS: u64 = 16;

pub struct ProjectilePlugin;

use crate::td_mode::enemy::CenterOfMass;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        let mut early_fixed_stage = SystemStage::parallel();
        let mut fixed_stage = SystemStage::parallel();
        let mut late_fixed_stage = SystemStage::parallel();
        let mut last_fixed_stage = SystemStage::parallel();
        early_fixed_stage.add_system(aim_homing_projectiles);
        fixed_stage.add_system(move_projectiles);
        late_fixed_stage.add_system(projectile_enemy_collisions);
        last_fixed_stage.add_system(progress_projectile_lifespans);

        app.add_startup_system(initialize_projectile_models)
            .add_system_to_stage(CoreStage::PreUpdate, handle_projectile_spawn_messages)
            .add_system(set_initial_direction)
            .add_system(set_initial_speed)
            .add_system_to_stage(CoreStage::PostUpdate, apply_spread)
            .add_system(add_homing)
            .add_stage_before(
                CoreStage::Update,
                "projectile_fixed_stage",
                FixedTimestepStage::new(std::time::Duration::from_millis(FIXED_STAGE_MS))
                    .with_stage(early_fixed_stage)
                    .with_stage(fixed_stage)
                    .with_stage(late_fixed_stage)
                    .with_stage(last_fixed_stage),
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
    models: &Res<ProjectileModels>,
    commands: &mut Commands,
    parent: Entity,
) -> Entity {
    commands
        .spawn()
        .insert(Speed(0.1))
        .insert(Projectile(ProjectileType::Ballista))
        .insert(Lifetime(Duration::from_secs(1)))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(start_position),
        ))
        .insert(Parent(parent))
        .with_children(|p| {
            p.spawn_scene(models.ballista_bolt.clone());
        })
        .id()
}

fn handle_projectile_spawn_messages(
    message_query: Query<(Entity, &Message, &Sender, &Target, &SpawnProjectile)>,
    spawn_point_query: Query<(&Transform, &ProjectileSpawnPoint)>,
    multishot_query: Query<&Multishot>,
    models: Res<ProjectileModels>,
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    message_query.iter().for_each(|(entity, _, sender, _, _)| {
        if let Ok((spawn_tform, spawn_point)) = spawn_point_query.get(sender.0) {
            let mut projectile_count = 1;
            if let Ok(multishot) = multishot_query.get(sender.0) {
                projectile_count = multishot.0;
            }

            let projectiles = (0..projectile_count)
                .map(|_| {
                    spawn_ballista_bolt(
                        spawn_tform.translation + spawn_point.0,
                        &models,
                        &mut commands,
                        current_map.0.unwrap(),
                    )
                })
                .collect();
            commands.entity(entity).insert(AlreadySpawned(projectiles));
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
                let proj_transform = transform_query.get(*e).unwrap();
                let target_transform = enemy_query.get(target.0).unwrap();
                let new_transform = proj_transform.looking_at(
                    target_transform.0.translation + target_transform.1 .0,
                    Vec3::Y,
                );
                commands.entity(*e).insert(new_transform);
            });
        });
}

fn aim_homing_projectiles(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Homing, &Target)>,
    transform_query: Query<(&Transform, &CenterOfMass)>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(entity, p_transform, _, _, target)| {
            if let Ok((target_transform, target_center_of_mass)) = transform_query.get(target.0) {
                let target_pos = target_transform.translation + target_center_of_mass.0;
                let new_transform = p_transform.looking_at(target_pos, Vec3::Y);

                commands.entity(entity).insert(new_transform);
            }
        });
}

fn move_projectiles(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Speed)>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(entity, p_transform, _, speed)| {
            let new_transform = p_transform;
            let new_transform = new_transform
                .with_translation(new_transform.translation + new_transform.forward() * speed.0);

            commands.entity(entity).insert(new_transform);
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

fn projectile_enemy_collisions(
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    enemy_query: Query<(Entity, &Transform, &CenterOfMass, &Enemy)>,
    mut commands: Commands,
) {
    projectile_query.iter().for_each(|(proj_e, proj_t, _)| {
        enemy_query
            .iter()
            .for_each(|(enemy_e, enemy_t, enemy_center, _)| {
                if proj_t
                    .translation
                    .distance(enemy_t.translation + enemy_center.0)
                    < 0.25
                {
                    commands.entity(proj_e).despawn_recursive();
                    commands.entity(enemy_e).despawn_recursive();
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
                let proj_transform = transform_query.get(*e).unwrap().clone();
                let forward_point = proj_transform.translation + proj_transform.forward();
                let offset = proj_transform.right() * thread_rng().gen_range(-spread.0..spread.0)
                    + proj_transform.up() * thread_rng().gen_range(-spread.0..spread.0);
                commands
                    .entity(*e)
                    .insert(proj_transform.looking_at(forward_point + offset, Vec3::Y));
            });
        });
}
