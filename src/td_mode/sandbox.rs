//! Code for the map editor/sandbox tools

use super::*;
use bevy_egui::{egui, EguiContext};

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SandboxControlState::new())
            .add_system(sandbox_ui.run_in_state(GameState::TDMode));
    }
}

struct SandboxControlState {
    new_dimensions: (usize, usize),
}

impl SandboxControlState {
    fn new() -> Self {
        Self {
            new_dimensions: (8, 8),
        }
    }
}

fn sandbox_ui(
    mut control_state: ResMut<SandboxControlState>,
    mut egui_context: ResMut<EguiContext>,
    mut map: ResMut<map::Map>,
) {
    egui::Window::new("Sandbox Tools").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Map");
        ui.label(format!("Current Size: {:?}", map.dimensions));
        ui.add(egui::Slider::new(
            &mut control_state.new_dimensions.0,
            1..=64,
        ));
        ui.add(egui::Slider::new(
            &mut control_state.new_dimensions.1,
            1..=64,
        ));

        if ui.button("Resize").clicked() {
            map.resize(control_state.new_dimensions);
        }
    });
}
