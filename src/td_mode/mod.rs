//! The Module for the Gameplay part of the game
//!
//! All code relating to the actual tower defense part of the game is in here.
//! In addition, in-game tools, examples, benchmarks, etc. are also part of this
//! module and its children.
use crate::prelude::*;

mod camera;

pub struct TDModePlugin;

impl Plugin for TDModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::TDCameraPlugin)
            .add_enter_system(GameState::TDMode, setup)
            .add_system(
                go_to_main_menu
                    .run_in_state(GameState::TDMode)
                    .run_if(escape_pressed),
            );
    }
}

/// Initialize a sample model and camera.
fn setup(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        ..default()
    });
}

fn escape_pressed(keys: Res<Input<KeyCode>>) -> bool {
    keys.pressed(KeyCode::Escape)
}

/// Delete all entities and return to the main menu.
fn go_to_main_menu(e_query: Query<Entity>, mut commands: Commands) {
    e_query.iter().for_each(|e| commands.entity(e).despawn());

    commands.insert_resource(NextState(GameState::MainMenu));
}
