use crate::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_tower_models)
            .add_system(detect_targets_in_range);

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
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(map.calculate_tile_pos(coord.x, coord.y))
                .with_scale(Vec3::ONE * 0.5),
        ))
        .with_children(|p| {
            p.spawn_scene(models.base.clone());
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
                        <= range.max_range
                })
                .map(|(e_entity, _, _)| e_entity)
                .collect();

            let valid_target_count = enemies_in_range.len();
            if valid_target_count > 0 {
                eprintln!("{valid_target_count}");
            }
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

fn add_debug_range_spheres(
    query: Query<(Entity, &Range), Added<Range>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    query.iter().for_each(|(entity, range)| {
        commands.entity(entity).with_children(|p| {
            p.spawn().insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: range.max_range,
                    sectors: 16,
                    stacks: 16,
                })),
                ..default()
            });
        });
    });
}
