//! Map and Tile code

mod map;
mod structures;
mod tile;

pub use super::td_mode_prelude::*;
use crate::prelude::*;

pub use map::*;
pub use structures::*;
pub use tile::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::empty())
            .add_plugin(TilePlugin)
            .add_plugin(StructuresPlugin)
            .add_system(
                update_changed_tiles
                    .run_in_state(GameState::TDMode)
                    .run_if(are_tiles_dirty),
            )
            .add_system(
                reload_all_map_tiles
                    .run_in_state(GameState::TDMode)
                    .run_if(is_map_resized),
            );
    }
}

fn reload_all_map_tiles(
    mut map_root_query: Query<(Entity, &mut MapRoot)>,
    mut map: ResMut<Map>,
    mut commands: Commands,
) {
    if map_root_query.is_empty() {
        commands
            .spawn()
            .insert_bundle(TransformBundle::identity())
            .insert(MapRoot::new())
            .insert(Name::new("Map"));
    } else {
        info!("Reloading all map tiles...");
        let (root_e, mut map_root) = map_root_query.single_mut();
        let existing_tile_count = map_root.tile_entities.len();

        if existing_tile_count < map.tile_count() {
            // Spawn tiles until you have enough.
            let tiles_to_spawn = map.tile_count() - existing_tile_count;
            let new_entities: Vec<Entity> = (0..tiles_to_spawn)
                .map(|_| commands.spawn().insert(Parent(root_e)).id())
                .collect();
            map_root
                .tile_entities
                .extend_from_slice(new_entities.as_slice());
        } else if existing_tile_count > map.tile_count() {
            // Despawn extra tiles
            let number_of_tiles_to_remove = existing_tile_count - map.tile_count();
            let idx_to_drain_from = existing_tile_count - number_of_tiles_to_remove;
            trace!("Despawning {number_of_tiles_to_remove} tiles starting from idx {idx_to_drain_from}.");
            map_root
                .tile_entities
                .drain(idx_to_drain_from..)
                .for_each(|e| commands.entity(e).despawn_recursive());

            trace!(
                "There are {} tiles remaining.",
                map_root.tile_entities.len()
            );
        }

        map_root
            .tile_entities
            .iter()
            .enumerate()
            .for_each(|(idx, e)| {
                let tile_type = map.tile_type_at_index(idx).unwrap();
                commands
                    .entity(*e)
                    .insert(Tile)
                    .insert(*tile_type)
                    .insert(map.idx_to_coord(idx))
                    .insert_bundle(TransformBundle::identity());
            });

        commands.entity(root_e).insert(Transform::from_xyz(
            map.dimensions.0 as f32 * -0.5,
            0.0,
            map.dimensions.1 as f32 * -0.5,
        ));

        map.size_dirty = false;
    }
}

fn update_changed_tiles(
    tiles: Query<(Entity, &Coordinate), With<Tile>>,
    mut map: ResMut<Map>,
    mut commands: Commands,
) {
    tiles
        .iter()
        .filter(|(_, coord)| map.dirty_tiles.contains(&map.coord_to_idx(**coord)))
        .for_each(|(e, coord)| {
            let tile_type = map.tile_type_at_coord(*coord).unwrap();

            commands.entity(e).insert(*tile_type);
        });

    map.dirty_tiles = Vec::new();
}

/// Tag Component for the parent transform for all of the map tiles
/// Contains a Vec of all the child tiles for better iteration
#[derive(Component)]
pub struct MapRoot {
    pub tile_entities: Vec<Entity>,
}

impl MapRoot {
    fn new() -> Self {
        Self {
            tile_entities: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_to_idx_on_small_map() {
        let map = Map::new((1, 2));

        let actual = map.coord_to_idx(Coordinate::from((0, 1)));
        let expected = 1;

        assert_eq!(actual, expected);
    }
}
