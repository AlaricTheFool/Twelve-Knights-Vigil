//! A 3D Tower Defense Game Starring 12 strange characters
//!
//! [X] Implement Swole Knight Minigame
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

#[cfg(feature = "debug")]
mod debug;

mod prelude {
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
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(debug::TKDebugPlugin);
    }

    app.run();
}
