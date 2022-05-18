use crate::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_tower_models);
    }
}

pub struct TowerModels {
    base: Handle<Scene>,
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
        .insert(Parent(map_entity))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(map.calculate_tile_pos(coord.x, coord.y))
                .with_scale(Vec3::ONE * 0.5),
        ))
        .with_children(|p| {
            p.spawn_scene(models.base.clone());
        });
}
