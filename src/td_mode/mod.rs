//! The Module for the Gameplay part of the game
//!
//! All code relating to the actual tower defense part of the game is in here.
//! In addition, in-game tools, examples, benchmarks, etc. are also part of this
//! module and its children.
use crate::prelude::*;

mod camera;
mod elements;
mod map;
mod messages;
mod raycast;
mod sandbox;

use elements::Element;
use messages::*;

pub struct TDModePlugin;

impl Plugin for TDModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::TDCameraPlugin)
            .add_plugin(map::MapPlugin)
            .add_plugin(raycast::PickablePlugin)
            .add_plugin(sandbox::SandboxPlugin)
            .add_plugin(elements::ElementPlugin)
            .add_enter_system(GameState::TDMode, setup)
            .add_system_to_stage(
                CoreStage::First,
                messages::clear_handled_messages.run_in_state(GameState::TDMode),
            )
            .add_system(
                go_to_main_menu
                    .run_in_state(GameState::TDMode)
                    .run_if(escape_pressed),
            );
    }
}

/// Spawn a directional light.
fn setup(mut commands: Commands) {
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::identity().with_rotation(Quat::from_euler(
            EulerRot::ZYX,
            -std::f32::consts::FRAC_PI_4,
            0.0,
            -std::f32::consts::FRAC_PI_4,
        )),
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
