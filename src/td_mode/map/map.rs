use super::*;

/// A resource containing a collection of tiles
pub struct Map {
    /// usize tuple, (width, height)
    pub dimensions: (usize, usize),

    /// A flag used to indicate that the size of the map has changed and that the number of tile
    /// entities needs to be updated
    pub size_dirty: bool,

    tiles: Vec<(TileType, Structure)>,

    /// Flag the index of tiles that have been edited so that the map's entities can be  updated.
    pub dirty_tiles: Vec<usize>,

    pub wave_entry_coord: Coordinate,
    pub wave_exit_coord: Coordinate,
}

impl Map {
    pub fn empty() -> Self {
        Self::new((0, 0))
    }

    pub fn is_empty(&self) -> bool {
        self.dimensions == (0, 0)
    }

    pub fn new(dimensions: (usize, usize)) -> Self {
        Self {
            dimensions,
            size_dirty: true,
            tiles: vec![(TileType::Barren, Structure::None); dimensions.0 * dimensions.1],
            dirty_tiles: Vec::new(),
            wave_entry_coord: Coordinate::ZERO,
            wave_exit_coord: Coordinate::ZERO,
        }
    }

    pub fn tile_count(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    pub fn resize(&mut self, new_dimensions: (usize, usize)) {
        self.dimensions = new_dimensions;
        self.tiles.resize(
            new_dimensions.0 * new_dimensions.1,
            (TileType::Barren, Structure::None),
        );
        self.size_dirty = true;
    }

    pub fn idx_to_coord(&self, idx: usize) -> Coordinate {
        let y = idx / self.dimensions.0;
        let x = idx - (self.dimensions.0 * y);
        (x, y).into()
    }

    pub fn coord_to_idx(&self, coord: Coordinate) -> usize {
        coord.y * self.dimensions.0 + coord.x
    }

    pub fn find_astar_successors(&self, coord: Coordinate) -> Vec<(Coordinate, u32)> {
        self.coord_cardinal_indices(coord)
            .iter()
            .map(|&idx| {
                (
                    self.idx_to_coord(idx),
                    self.tile_type_at_index(idx).unwrap().astar_cost(),
                )
            })
            .collect()
    }

    pub fn tile_type_at_index(&self, idx: usize) -> Option<&TileType> {
        self.tiles.get(idx).map(|(t_type, _)| t_type)
    }

    pub fn structure_at_index(&self, idx: usize) -> Option<&Structure> {
        self.tiles.get(idx).map(|(_, structure)| structure)
    }

    pub fn set_tile(
        &mut self,
        coord: Coordinate,
        new_tile_type: Option<TileType>,
        new_structure: Option<Structure>,
    ) {
        let idx = self.coord_to_idx(coord);

        let t_type = if let Some(tile_type) = new_tile_type {
            tile_type
        } else {
            *self.tile_type_at_index(idx).unwrap()
        };

        let structure = if let Some(structure) = new_structure {
            structure
        } else {
            *self.structure_at_index(idx).unwrap()
        };

        self.tiles[idx] = (t_type, structure);
        self.dirty_tiles.push(idx);
    }

    pub fn coord_cardinal_indices(&self, coord: Coordinate) -> Vec<usize> {
        coord
            .cardinals()
            .iter()
            .filter_map(move |&c| {
                if self.coord_in_bounds(c) {
                    Some(self.coord_to_idx(c))
                } else {
                    None
                }
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

    pub fn structure_at_coord(&self, coord: Coordinate) -> Option<&Structure> {
        let idx = self.coord_to_idx(coord);
        self.structure_at_index(idx)
    }
}

pub fn is_map_resized(map: Res<Map>) -> bool {
    map.size_dirty
}

pub fn are_tiles_dirty(map: Res<Map>) -> bool {
    !map.dirty_tiles.is_empty()
}
