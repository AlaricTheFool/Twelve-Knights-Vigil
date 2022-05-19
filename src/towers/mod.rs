use crate::prelude::*;

mod cooldown;
mod weapons;

pub struct TowerPlugin;
pub use self::cooldown::{spawn_cd_reset_message, Cooldown};
use self::weapons::*;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_tower_models)
            .add_plugin(cooldown::CDPlugin)
            .add_system(detect_targets_in_range)
            .add_system_to_stage(CoreStage::Last, fire_weapons);

        #[cfg(feature = "debug")]
        {
            //app.add_system(add_debug_range_spheres);
        }
    }
}

pub struct TowerModels {
    base: Handle<Scene>,
    ballista: Handle<Scene>,
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
        ballista: assets.load("models/towers/weapons/weapon_ballista.glb#Scene0"),
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
        })
        .with_children(|p| {
            p.spawn()
                .insert_bundle(TransformBundle::from_transform(
                    Transform::from_translation(Vec3::new(0.0, 0.7, 0.0)),
                ))
                .insert(WeaponPivot)
                .with_children(|p| {
                    p.spawn_scene(models.ballista.clone());
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

fn fire_weapons(
    mut query: Query<(Entity, &Weapon, &Cooldown, &ValidTargets)>,
    mut track_follower_query: Query<(Entity, &TrackFollower, &Transform)>,
    mut pivot_query: Query<(Entity, &Transform, &GlobalTransform, &WeaponPivot, &Parent)>,
    mut commands: Commands,
) {
    query.iter_mut().for_each(|(entity, _, cd, targets)| {
        if targets.valid_targets.len() > 0 && cd.is_ready() {
            let mut target = track_follower_query
                .iter()
                .filter(|(e, _, _)| targets.valid_targets.contains(e))
                .collect::<Vec<(Entity, &TrackFollower, &Transform)>>();

            target.sort_by(|b, a| a.1.progress.partial_cmp(&b.1.progress).unwrap());

            pivot_query
                .iter()
                .filter(|(_, _, _, _, parent)| parent.0 == entity)
                .for_each(|(pivot_entity, transform, g_transform, _, _)| {
                    let target_pos = target[0].2.translation.flatten();
                    let angle = calculate_point_at_angle(g_transform.translation.flatten(), target_pos);
                        
                    let rotation = Quat::from_euler(EulerRot::XYZ, 0.0, angle, 0.0);
                    let modified_transform = transform.with_rotation(rotation);
                    eprintln!("Angle is {angle:?} and Rotation is {rotation:?} and transform is {modified_transform:?}");
                    commands
                        .entity(pivot_entity)
                        .insert_bundle(TransformBundle::from_transform(Transform::from(modified_transform)));
                });

            commands.entity(target[0].0).despawn_recursive();
            spawn_cd_reset_message(entity, &mut commands);
        }
    });
}

fn calculate_point_at_angle(source: Vec2, target: Vec2) -> f32 {
    eprintln!("The source is {source} and it's pointing at {target}");
    source.angle_between(target) - std::f32::consts::FRAC_PI_2
}
