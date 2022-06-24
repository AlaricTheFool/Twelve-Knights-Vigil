//! Code for initializing models and updating tiles in a map

use super::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_tile_positions)
            .add_system(update_tile_model);
    }
}

/// Tag component for tiles
#[derive(Component)]
pub struct Tile;

#[derive(Copy, Clone, Component)]
pub enum TileType {
    Rock,
    /*
    Empty,
    Water,
    Fire,
    */
}

/// Container resource for tile models
struct TileModels {
    rock: Handle<Scene>,
}

#[derive(Component)]
struct ModelRoot(Entity);

/// Load tile models from disk into a TileModels struct
fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let tile_models = TileModels {
        rock: assets.load("models/tile.glb#Scene0"),
    };

    commands.insert_resource(tile_models);
}

/// Sets the tile's position using its grid coordinate and an assumed size of 1.0
///
/// NOTE: This is called when the ModelRoot is changed as well to force a transform
/// propogation.
fn update_tile_positions(
    tile_query: Query<
        (Entity, &Coordinate),
        (With<Tile>, Or<(Changed<Coordinate>, Changed<ModelRoot>)>),
    >,
    mut commands: Commands,
) {
    tile_query.iter().for_each(|(e, coord)| {
        let tlation = *coord * Vec3::new(1.0, 0.0, 1.0);
        commands
            .entity(e)
            .insert(Transform::from_translation(tlation));
    });
}

fn update_tile_model(
    tile_query: Query<(Entity, &TileType, Option<&ModelRoot>), (With<Tile>, Changed<TileType>)>,
    models: Res<TileModels>,
    mut commands: Commands,
) {
    tile_query.iter().for_each(|(e, tile_type, existing_root)| {
        if let Some(existing_root) = existing_root {
            commands.entity(existing_root.0).despawn_recursive();
        }

        let new_root_e = commands
            .spawn()
            .insert(Parent(e))
            .insert_bundle(TransformBundle::identity())
            .with_children(|p| {
                // SPAWN MODEL HERE
                p.spawn_scene(models.rock.clone());
            })
            .id();
        commands.entity(e).insert(ModelRoot(new_root_e));
    });
}
