//! A 3D Tower Defense Game Starring 12 strange characters
//!
//! Twelve Knight's Vigil is a Tower Defense game in which you control 12
//! different Knights and move them between towers. This is a free-pathing
//! tower-defense, meaning enemies don't have a static path that they follow.
//! Instead, enemies path dynamically through a map based on the easiest terrain,
//! and the player can affect this map by erecting barricades and changing the terrain.
//!
//! There are 3 basic points of complexity in the gameplay:
//! * The Elemental Chemistry System - There are 8 different elemental effects that can be
//! applied to enemies, terrain, and towers, creating different effects based on what's mixed
//! with what. For example, a  fire attack on a forest terrain may light the tile on fire.
//!
//! * The Terrain System - Maps are composed of tiles and each tile has an associated terrain type.
//! Terrain is affected by the abilities of enemies and the player, as well as by other terrain.
//! For example, a burning forest will spread fire to adjacent forests. Watering a burnt out forest
//! will regrow it.
//!
//! * The Enemy Movement System - There are various ways enemies can move around the map or be forcefully
//! moved by the player. For example, an enemy may open a portal to cross a barricade or a tower
//! may send an enemy flying across the map.

#![warn(clippy::missing_docs, clippy::all, clippy::pedantic)]

mod debug;

mod prelude {
    pub use bevy::prelude::*;
}

use crate::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Twelve Knight's Vigil".to_string(),
        width: 1280.0,
        height: 720.0,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(debug::TKDebugPlugin);
    }

    app.run();
}
