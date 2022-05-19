use crate::prelude::*;

mod cooldown;

pub struct TowerPlugin;
pub use self::cooldown::{spawn_cd_reset_message, Cooldown};

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_tower_models)
            .add_plugin(cooldown::CDPlugin)
            .add_system(detect_targets_in_range)
            .add_system_to_stage(CoreStage::Last, fire_weapons);

        #[cfg(feature = "debug")]
        {
            app.add_system(add_debug_range_spheres);
        }
    }
}

pub struct TowerModels {
    base: Handle<Scene>,
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
pub struct Weapon;

#[derive(Component)]
pub struct ValidTargets {
    pub valid_targets: Vec<Entity>,
}

fn initialize_tower_models(assets: Res<AssetServer>, mut commands: Commands) {
    let tower_models = TowerModels {
        base: assets.load("models/towers/towerSquare_sampleA.glb#Scene0"),
    };

    commands.insert_resource(tower_models);
}

pub fn spawn_tower(
    map_entity: Entity,
    map: &TileMap,
    coord: Coordinate,
    commands: &mut Commands,
    models: &TowerModels,
) {
    eprintln!("Spawning tower at {coord:?}");
    commands
        .spawn()
        .insert(Name::new(format!("Tower [{}, {}]", coord.x, coord.y)))
        .insert(Parent(map_entity))
        .insert(Range { max_range: 1.0 })
        .insert(Weapon)
        .insert(Cooldown::new(0.5))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(map.calculate_tile_pos(coord.x, coord.y)),
        ))
        .with_children(|p| {
            p.spawn()
                .insert_bundle(TransformBundle::from_transform(
                    Transform::identity().with_scale(Vec3::ONE * 0.5),
                ))
                .with_children(|p| {
                    p.spawn_scene(models.base.clone());
                });
        });
}

fn detect_targets_in_range(
    range_query: Query<(Entity, &Transform, &Range)>,
    enemy_query: Query<(Entity, &Enemy, &Transform)>,
    mut commands: Commands,
) {
    range_query
        .iter()
        .for_each(|(r_entity, r_transform, range)| {
            let enemies_in_range: Vec<Entity> = enemy_query
                .iter()
                .filter(|(_, _, e_transform)| {
                    r_transform
                        .translation
                        .flatten()
                        .distance(e_transform.translation.flatten())
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
    type Output = Self;

    fn flatten(&self) -> Self::Output {
        *self * Vec3::new(1.0, 0.0, 1.0)
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

fn fire_weapons(
    mut query: Query<(Entity, &Weapon, &Cooldown, &ValidTargets)>,
    mut track_follower_query: Query<(Entity, &TrackFollower)>,
    mut commands: Commands,
) {
    query.iter_mut().for_each(|(entity, _, cd, targets)| {
        if targets.valid_targets.len() > 0 && cd.is_ready() {
            let mut target = track_follower_query
                .iter()
                .filter(|(e, _)| targets.valid_targets.contains(e))
                .collect::<Vec<(Entity, &TrackFollower)>>();

            target.sort_by(|b, a| a.1.progress.partial_cmp(&b.1.progress).unwrap());

            commands.entity(target[0].0).despawn_recursive();
            spawn_cd_reset_message(entity, &mut commands);
        }
    });
}
