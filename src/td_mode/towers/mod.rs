use super::*;

mod building;
mod cooldown;
mod projectiles;
mod weapons;

pub struct TowerPlugin;
pub use self::cooldown::{spawn_cd_reset_message, Cooldown};
use self::projectiles::*;
use self::{projectiles::spawn_projectile_message, weapons::*};
pub use building::*;

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
            .add_system_to_stage(
                CoreStage::PostUpdate,
                damage_targeted_enemy.run_in_state(GameMode::TDMode),
            );

        #[cfg(feature = "debug")]
        {
            //app.add_system(add_debug_range_spheres);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
pub struct Range {
    pub max_range: f32,
}

impl Range {
    pub fn calculate_adjusted_range(&self) -> f32 {
        self.max_range + 0.5
    }
}

#[derive(Component)]
pub struct CurrentTarget(Entity);

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct ValidTargets {
    pub valid_targets: Vec<Entity>,
}

#[derive(Component)]
pub struct ProjectileSpawnPoint(Vec3);

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
    eprintln!("Spawning tower at {coord:?}");
    let tower_entity = commands
        .spawn()
        .insert(Name::new(format!("Tower [{}, {}]", coord.x, coord.y)))
        .insert(Parent(map_entity))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(map.calculate_tile_pos(coord.x, coord.y)),
        ))
        .with_children(|p| {
            p.spawn()
                .insert_bundle(TransformBundle::from_transform(
                    Transform::identity()
                        .with_scale(Vec3::ONE * 0.5)
                        .with_translation(Vec3::Y * 0.25),
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
                        <= range.calculate_adjusted_range()
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
                    radius: range.calculate_adjusted_range(),
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

fn damage_targeted_enemy(
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
