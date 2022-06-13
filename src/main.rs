//! A 3D Tower Defense Game Starring 12 strange characters
//!
//! [ ] Implement Swole Knight Minigame
//! [ ] Implement Normal Knight Tall Tower
//! [X] Design Swole Knight Towers
//!     Short: Shove -> Damage and Big Knockback
//!     Medium: Big Fist Smack Ground -> AOE damage and weaker (relative to short) knockback
//!     Tall: Set a target location on the map. Pick up single enemies and throw em. (Distance
//!     based on power bar.)
//! [ ] Design Lizard Knight Minigame
//! [ ] Implement Lizard Knight Minigame
//!
//! [ ] Make a Discord Bot that controls the game.

#![warn(clippy::missing_docs, clippy::all, clippy::pedantic)]

mod cheats;
mod coordinate;
mod gamemode;
mod input;
mod knights;
mod main_menu;
mod messages;
mod td_mode;
mod tilemap;
mod vn_mode;

#[cfg(feature = "debug")]
mod debug;

mod prelude {
    pub use crate::cheats::*;
    pub use crate::coordinate::*;
    pub use crate::gamemode::*;
    pub use crate::input::*;
    pub use crate::knights::*;
    pub use crate::main_menu::*;
    pub use crate::messages::*;
    pub use crate::td_mode::*;
    pub use crate::tilemap::*;
    pub use crate::vn_mode::*;
    pub use bevy::input::mouse::*;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContext, EguiPlugin};
    pub use bevy_mod_raycast::{RayCastMesh, RayCastSource};
    pub use iyes_loopless::prelude::*;
    pub use rand::*;

    #[cfg(feature = "debug")]
    pub use debug;

    #[cfg(feature = "debug")]
    pub use bevy_inspector_egui::RegisterInspectable;
}

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Twelve Knight's Vigil".to_string(),
        width: 1280.0,
        height: 720.0,
        //        present_mode: bevy::window::PresentMode::Immediate,
        ..default()
    })
    .add_loopless_state(GameMode::MainMenu)
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(debug::TKDebugPlugin);
    }

    app.add_plugin(MessagePlugin).add_plugin(InputPlugin);

    app.add_plugin(TDModePlugin)
        .add_plugin(VNModePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(cheats::CheatPlugin);

    /*
    app.add_plugin(PickablePlugin)

        .add_plugin(TowerPlugin)
    */

    app.run();
}
