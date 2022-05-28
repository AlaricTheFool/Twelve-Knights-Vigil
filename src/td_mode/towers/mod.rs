use super::*;

mod building;
mod cooldown;
mod projectiles;
mod weapons;

pub struct TowerPlugin;
pub use self::cooldown::{spawn_cd_reset_message, Cooldown};
use self::projectiles::*;
pub use building::*;
pub use weapons::*;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_tower_models)
            .add_plugin(cooldown::CDPlugin)
            .add_plugin(projectiles::ProjectilePlugin)
            .add_system(
                detect_targets_in_range
                    .run_in_state(GameMode::TDMode)
                    .label("detect_target"),
            )
            .add_system(
                update_current_target
                    .run_in_state(GameMode::TDMode)
                    .after("detect_target"),
            )
            .add_system(point_weapons_at_targets.run_in_state(GameMode::TDMode))
            .add_system(handle_build_tower_messages.run_in_state(GameMode::TDMode))
            .add_system(handle_place_knight_messages.run_in_state(GameMode::TDMode))
            .add_system(normal_knight_slider_changed.run_in_state(GameMode::TDMode))
            .add_system(sell_towers.run_in_state(GameMode::TDMode))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                fire_projectiles_at_targets.run_in_state(GameMode::TDMode),
            );

        #[cfg(feature = "debug")]
        {
            //app.add_system(add_debug_range_spheres);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub enum TowerType {
    Short,
    Medium,
    Tall,
}

pub struct TowerModels {
    base: Handle<Scene>,
    short: Handle<Scene>,
    med: Handle<Scene>,
    tall: Handle<Scene>,
}

#[derive(Component)]
pub struct Sell;

#[derive(Component, Copy, Clone)]
pub struct Range(f32);

impl Range {
    pub fn new(r: f32) -> Self {
        Self(r)
    }
    pub fn get_adjusted(&self) -> f32 {
        self.0 + 0.5
    }
}

#[derive(Component)]
pub struct Tower;

#[derive(Component)]
pub struct CurrentTarget(Entity);

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct ValidTargets {
    pub valid_targets: Vec<Entity>,
}

#[derive(Component)]
pub struct ProjectileSpawnPoint(pub Vec3);

fn initialize_tower_models(assets: Res<AssetServer>, mut commands: Commands) {
    let tower_models = TowerModels {
        base: assets.load("models/towers/towerSquare_sampleA.glb#Scene0"),
        short: assets.load("models/towers/tower_short.glb#Scene0"),
        med: assets.load("models/towers/tower_medium.glb#Scene0"),
        tall: assets.load("models/towers/tower_tall.glb#Scene0"),
    };

    commands.insert_resource(tower_models);
}

pub fn spawn_tower(
    map_entity: Entity,
    map: &TileMap,
    coord: Coordinate,
    commands: &mut Commands,
    models: &TowerModels,
    t_type: TowerType,
) {
    info!("Spawning tower at {coord:?}");
    let tower_entity = commands
        .spawn()
        .insert(Name::new(format!("Tower [{}, {}]", coord.x, coord.y)))
        .insert(coord)
        .insert(Parent(map_entity))
        .insert(t_type)
        .insert(Tower)
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(map.calculate_tile_pos(coord.x, coord.y)),
        ))
        .with_children(|p| {
            p.spawn()
                .insert_bundle(TransformBundle::from_transform(
                    Transform::identity()
                        .with_scale(Vec3::ONE * 0.5)
                        .with_translation(Vec3::Y * 0.20),
                ))
                .with_children(|p| {
                    let model = match t_type {
                        TowerType::Short => models.short.clone(),
                        TowerType::Medium => models.med.clone(),
                        TowerType::Tall => models.tall.clone(),
                    };
                    p.spawn_scene(model);
                });
        })
        .id();

    let pivot_entity = commands
        .spawn()
        .insert(Parent(tower_entity))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, 0.7, 0.0)),
        ))
        .id();

    commands
        .entity(tower_entity)
        .insert(WeaponPivot(pivot_entity));
}

fn detect_targets_in_range(
    range_query: Query<(Entity, &GlobalTransform, &Range, &Name)>,
    enemy_query: Query<(Entity, &Enemy, &GlobalTransform)>,
    mut commands: Commands,
) {
    range_query
        .iter()
        .for_each(|(r_entity, r_transform, range, name)| {
            let enemies_in_range: Vec<Entity> = enemy_query
                .iter()
                .filter(|(_, _, e_transform)| {
                    r_transform.translation.distance(e_transform.translation)
                        <= range.get_adjusted()
                })
                .map(|(e_entity, _, _)| e_entity)
                .collect();

            commands.entity(r_entity).insert(ValidTargets {
                valid_targets: enemies_in_range,
            });
        })
}

trait Flatten {
    type Output;
    fn flatten(&self) -> Self::Output;
}

impl Flatten for Vec3 {
    type Output = Vec2;

    fn flatten(&self) -> Self::Output {
        Vec2::new(self.x, self.z)
    }
}

#[cfg(feature = "debug")]
fn add_debug_range_spheres(
    query: Query<(Entity, &Range), Added<Range>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    debug_models: Res<crate::debug::DebugModels>,
) {
    query.iter().for_each(|(entity, range)| {
        commands.entity(entity).with_children(|p| {
            p.spawn().insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: range.get_adjusted(),
                    sectors: 16,
                    stacks: 16,
                })),
                material: debug_models.debug_material.clone(),
                ..default()
            });
        });
    });
}

fn update_current_target(
    tower_query: Query<(Entity, &Weapon, &ValidTargets)>,
    track_follower_query: Query<(Entity, &TrackFollower)>,
    mut commands: Commands,
) {
    tower_query.iter().for_each(|(tower_entity, _, targets)| {
        let mut existing_valid_targets = track_follower_query
            .iter()
            .filter(|(e, _)| targets.valid_targets.contains(e))
            .collect::<Vec<(Entity, &TrackFollower)>>();
        if existing_valid_targets.len() > 0 {
            existing_valid_targets.sort_by(|b, a| a.1.progress.partial_cmp(&b.1.progress).unwrap());
            commands
                .entity(tower_entity)
                .insert(CurrentTarget(existing_valid_targets[0].0));
        } else {
            commands.entity(tower_entity).remove::<CurrentTarget>();
        }
    });
}

fn point_weapons_at_targets(
    transform_query: Query<(&Transform, &GlobalTransform)>,
    tower_query: Query<(&CurrentTarget, &WeaponPivot)>,
    mut commands: Commands,
) {
    tower_query.iter().for_each(|(target, pivot)| {
        // TODO: FIGURE OUT HOW TO QUATERNION.
    });
}

fn fire_projectiles_at_targets(
    tower_query: Query<(Entity, &CurrentTarget, &Cooldown)>,
    mut commands: Commands,
) {
    tower_query
        .iter()
        .filter(|(_, _, cooldown)| cooldown.is_ready())
        .for_each(|(tower_entity, target, _)| {
            spawn_projectile_message(
                tower_entity,
                target.0,
                ProjectileType::Ballista,
                &mut commands,
            );

            spawn_cd_reset_message(tower_entity, &mut commands);
        });
}

fn normal_knight_slider_changed(
    tower_query: Query<
        (Entity, &Tower, &Knight, &Cooldown, &Damage, &PowerSlider),
        Changed<PowerSlider>,
    >,
    mut commands: Commands,
) {
    tower_query
        .iter()
        .filter(|(_, _, knight, _, _, _)| **knight == Knight::Normal)
        .for_each(|(e, _, _, _, _, slider)| {
            let speed_pct = slider.get();
            let power_pct = slider.get_reverse();

            let min_cd = 0.1;
            let max_cd = 1.0;

            let new_cd = max_cd + ((min_cd - max_cd) * speed_pct);
            commands.entity(e).insert(Cooldown::new(new_cd));

            let min_damage = 1;
            let max_damage = 10;

            let new_damage =
                (min_damage as f32 + ((max_damage - min_damage) as f32 * power_pct).round()) as u32;
            commands.entity(e).insert(Damage(new_damage));
        });
}

fn sell_towers(
    sell_messages: Query<(Entity, &Target), (With<Message>, With<Sell>)>,
    knight_query: Query<&Knight>,
    mut commands: Commands,
    mut knight_statuses: ResMut<KnightStatuses>,
) {
    sell_messages.iter().for_each(|(message_e, target)| {
        if let Ok(knight) = knight_query.get(target.0) {
            knight_statuses.set_status(*knight, KUsageStatus::Ready);
        }

        commands.entity(target.0).despawn_recursive();
        commands.entity(message_e).insert(IsHandled);
    });
}
