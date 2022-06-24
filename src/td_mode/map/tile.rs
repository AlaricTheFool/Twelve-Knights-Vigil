//! Code for initializing models and updating tiles in a map

use super::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_tile_positions);
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

fn update_tile_positions(
    tile_query: Query<(Entity, &Coordinate), (With<Tile>, Changed<Coordinate>)>,
    mut commands: Commands,
) {
    tile_query.iter().for_each(|(e, coord)| {
        let tlation = *coord * Vec3::new(1.0, 0.0, 1.0);
        commands
            .entity(e)
            .insert(Transform::from_translation(tlation));
    });
}
