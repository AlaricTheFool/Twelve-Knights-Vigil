//! Map and Tile code

mod tile;

pub use super::td_mode_prelude::*;
use crate::prelude::*;

pub use tile::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::empty())
            .add_plugin(TilePlugin)
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

/// A resource containing a collection of tiles
pub struct Map {
    /// usize tuple, (width, height)
    pub dimensions: (usize, usize),

    /// A flag used to indicate that the size of the map has changed and that the number of tile
    /// entities needs to be updated
    size_dirty: bool,

    tiles: Vec<TileType>,

    /// Flag the index of tiles that have been edited so that the map's entities can be  updated.
    dirty_tiles: Vec<usize>,
}

impl Map {
    fn empty() -> Self {
        Self::new((0, 0))
    }

    fn new(dimensions: (usize, usize)) -> Self {
        Self {
            dimensions,
            size_dirty: true,
            tiles: vec![TileType::Rock; dimensions.0 * dimensions.1],
            dirty_tiles: Vec::new(),
        }
    }

    fn tile_count(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    pub fn resize(&mut self, new_dimensions: (usize, usize)) {
        self.dimensions = new_dimensions;
        self.tiles
            .resize(new_dimensions.0 * new_dimensions.1, TileType::Rock);
        self.size_dirty = true;
    }

    fn idx_to_coord(&self, idx: usize) -> Coordinate {
        let y = idx / self.dimensions.0;
        let x = idx - (self.dimensions.0 * y);
        (x, y).into()
    }

    pub fn coord_to_idx(&self, coord: Coordinate) -> usize {
        coord.y * self.dimensions.0 + coord.x
    }

    fn tile_type_at_index(&self, idx: usize) -> Option<&TileType> {
        self.tiles.get(idx)
    }

    pub fn set_tile(&mut self, coord: Coordinate, tile_type: TileType) {
        let idx = self.coord_to_idx(coord);
        self.tiles[idx] = tile_type;
        self.dirty_tiles.push(idx);
    }

    pub fn coord_adjacent_indices(&self, coord: Coordinate) -> Vec<usize> {
        let start_x = coord.x.saturating_sub(1);
        let end_x = (coord.x + 2).min(self.dimensions.0);

        let start_y = coord.y.saturating_sub(1);
        let end_y = (coord.y + 2).min(self.dimensions.1);

        (start_y..end_y)
            .flat_map(|y| {
                (start_x..end_x).filter_map(move |x| {
                    let this_coord = Coordinate::from((x, y));
                    if self.coord_in_bounds(this_coord) && this_coord != coord {
                        Some(self.coord_to_idx(this_coord))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn coord_in_bounds(&self, coord: Coordinate) -> bool {
        coord.x < self.dimensions.0 && coord.y < self.dimensions.1
    }

    pub fn tile_type_at_coord(&self, coord: Coordinate) -> Option<&TileType> {
        let idx = self.coord_to_idx(coord);
        self.tile_type_at_index(idx)
    }
}

fn is_map_resized(map: Res<Map>) -> bool {
    map.size_dirty
}

fn are_tiles_dirty(map: Res<Map>) -> bool {
    !map.dirty_tiles.is_empty()
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
            let idx = map.coord_to_idx(*coord);
            let tile_type = map.tiles[idx];

            commands.entity(e).insert(tile_type);
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
