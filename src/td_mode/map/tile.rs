//! Code for initializing models and updating tiles in a map

use super::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            // These systems have to be in PreUpdate because the Scene Spawner waits
            // until the end of PreUpdate to spawn its stuff. Changing these system's stage
            // introduces bugs due to ordering.
            .add_system_to_stage(CoreStage::PreUpdate, update_tile_positions)
            .add_system_to_stage(CoreStage::PreUpdate, update_tile_model);
    }
}

/// Tag component for tiles
#[derive(Component)]
pub struct Tile;

#[derive(Copy, Clone, Component, PartialEq, Debug)]
pub enum TileType {
    Rock,
    Water,
    Air,
    Fire,
    Barren,
}

impl TileType {
    pub fn all() -> [TileType; 5] {
        [
            TileType::Rock,
            TileType::Water,
            TileType::Air,
            TileType::Fire,
            TileType::Barren,
        ]
    }

    fn display_name(&self) -> &str {
        match *self {
            TileType::Rock => "Rock",
            TileType::Water => "Water",
            TileType::Air => "Air",
            TileType::Fire => "Fire",
            TileType::Barren => "Barren",
        }
    }

    pub fn astar_cost(&self) -> u32 {
        match *self {
            TileType::Water | TileType::Fire => 100,
            TileType::Air => 9999,
            TileType::Barren => 1,
            _ => 5,
        }
    }
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Container resource for tile models
struct TileModels {
    rock: Handle<Scene>,
    water: Handle<Scene>,
    air: Handle<Scene>,
    fire: Handle<Scene>,
    barren: Handle<Scene>,
}

impl TileModels {
    fn model_for_type(&self, t_type: TileType) -> Handle<Scene> {
        match t_type {
            TileType::Rock => self.rock.clone(),
            TileType::Water => self.water.clone(),
            TileType::Air => self.air.clone(),
            TileType::Fire => self.fire.clone(),
            TileType::Barren => self.barren.clone(),
        }
    }
}

/// Load tile models from disk into a TileModels struct
fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let tile_models = TileModels {
        rock: assets.load("models/tile_rock.glb#Scene0"),
        water: assets.load("models/tile_water.glb#Scene0"),
        air: assets.load("models/tile_air.glb#Scene0"),
        fire: assets.load("models/tile_fire.glb#Scene0"),
        barren: assets.load("models/tile_barren.glb#Scene0"),
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
                p.spawn_scene(models.model_for_type(*tile_type));
            })
            .id();

        commands.entity(e).insert(ModelRoot(new_root_e));
        trace!("Spawned the models for tile entity: {e:?}");
    });
}
