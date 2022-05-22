mod coordinate;
mod gamemode;
mod input;
mod main_menu;
mod messages;
mod td_mode;
mod tilemap;
mod towers;

#[cfg(feature = "debug")]
mod debug;

mod prelude {
    pub use crate::coordinate::*;
    pub use crate::gamemode::*;
    pub use crate::input::*;
    pub use crate::main_menu::*;
    pub use crate::messages::*;
    pub use crate::td_mode::*;
    pub use crate::tilemap::*;
    pub use crate::towers::*;
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
        present_mode: bevy::window::PresentMode::Mailbox,
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

    app.add_plugin(TDModePlugin).add_plugin(MainMenuPlugin);

    /*
    app.add_plugin(PickablePlugin)

        .add_plugin(TowerPlugin)
    */

    app.run();
}
