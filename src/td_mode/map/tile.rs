//! Code for initializing models and updating tiles in a map

use crate::td_mode::elements::ElementalAffliction;

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

        app.insert_resource(TileInteractionTimer(Timer::from_seconds(0.1, true)))
            .add_system(
                spread_fire
                    .run_in_state(GameState::TDMode)
                    .run_if(interaction_timer_complete),
            );
    }
}

struct TileInteractionTimer(Timer);

fn interaction_timer_complete(time: Res<Time>, mut timer: ResMut<TileInteractionTimer>) -> bool {
    timer.0.tick(time.delta()).just_finished()
}

/// Tag component for tiles
#[derive(Component)]
pub struct Tile;

#[derive(Copy, Clone, Component, PartialEq, Debug)]
pub enum TileType {
    Rock,
    Water,
    Empty,
    Fire,
}

impl TileType {
    pub fn all() -> [TileType; 4] {
        [
            TileType::Rock,
            TileType::Water,
            TileType::Empty,
            TileType::Fire,
        ]
    }

    fn display_name(&self) -> &str {
        match *self {
            TileType::Rock => "Rock",
            TileType::Water => "Water",
            TileType::Empty => "Air",
            TileType::Fire => "Fire",
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
    empty: Handle<Scene>,
    fire: Handle<Scene>,
}

impl TileModels {
    fn model_for_type(&self, t_type: TileType) -> Handle<Scene> {
        match t_type {
            TileType::Rock => self.rock.clone(),
            TileType::Water => self.water.clone(),
            TileType::Empty => self.empty.clone(),
            TileType::Fire => self.fire.clone(),
        }
    }
}

/// Load tile models from disk into a TileModels struct
fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let tile_models = TileModels {
        rock: assets.load("models/tile_rock.glb#Scene0"),
        water: assets.load("models/tile_water.glb#Scene0"),
        empty: assets.load("models/tile_empty.glb#Scene0"),
        fire: assets.load("models/tile_fire.glb#Scene0"),
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

fn spread_fire(
    tile_query: Query<(Entity, &TileType, &Coordinate, &ElementalAffliction), With<Tile>>,
    map_root_query: Query<&MapRoot>,
    map: Res<Map>,
    mut commands: Commands,
) {
    if let Ok(tiles) = map_root_query.get_single() {
        tile_query
            .iter()
            .filter(|(_, t_type, _, _)| **t_type == TileType::Fire)
            .for_each(|(entity, _, coord, elemental_affliction)| {
                let mut indices = map.coord_adjacent_indices(*coord);
                let remaining_fire = elemental_affliction.get_element_amount(Element::Fire);
                trace!("Remaining Fire: {remaining_fire}");
                indices.truncate(remaining_fire as usize);

                let mut actual_added = 0;
                indices.iter().for_each(|&idx| {
                    trace!("Adding one heat to tile at idx {idx}");
                    // ADD FIRE TO INDEX TILE
                    actual_added += 1;
                    let target_tile = tiles.tile_entities[idx];
                    commands
                        .spawn()
                        .insert(Message)
                        .insert(Target(target_tile))
                        .insert(ApplyElement)
                        .insert(ElementalAffliction::single(Element::Fire, 1));
                });

                // REMOVE FIRE FROM SOURCE TILE
                let amount_to_remove = indices.len();
                trace!("Removing {amount_to_remove} heat from source tile.");
                commands
                    .spawn()
                    .insert(Message)
                    .insert(Target(entity))
                    .insert(RemoveElements)
                    .insert(ElementalAffliction::single(
                        Element::Fire,
                        amount_to_remove as u32,
                    ));
            });
    }
}
