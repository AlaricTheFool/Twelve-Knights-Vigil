//! Map and Tile code

mod tile;

use crate::prelude::*;

pub use tile::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::empty())
            .add_plugin(TilePlugin)
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

    fn tile_type_at_index(&self, idx: usize) -> Option<&TileType> {
        self.tiles.get(idx)
    }
}

fn is_map_resized(map: Res<Map>) -> bool {
    map.size_dirty
}

fn reload_all_map_tiles(
    mut map_root_query: Query<(Entity, &mut MapRoot)>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                    .insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.9 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        ..default()
                    });
            });

        map.size_dirty = false;
    }
}

/// Tag Component for the parent transform for all of the map tiles
/// Contains a Vec of all the child tiles for better iteration
#[derive(Component)]
struct MapRoot {
    tile_entities: Vec<Entity>,
}

impl MapRoot {
    fn new() -> Self {
        Self {
            tile_entities: Vec::new(),
        }
    }
}
