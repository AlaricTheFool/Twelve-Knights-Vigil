//! Code for the map editor/sandbox tools

use super::*;
use bevy_egui::{egui, EguiContext};
use map::TileType;

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SandboxControlState::new())
            .add_system(sandbox_ui.run_in_state(GameState::TDMode))
            .add_system(paint_tile.run_in_state(GameState::TDMode));
    }
}

struct SandboxControlState {
    new_dimensions: (usize, usize),
    tile_brush: TileType,
}

impl SandboxControlState {
    fn new() -> Self {
        Self {
            new_dimensions: (8, 8),
            tile_brush: TileType::Water,
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

        ui.heading("Tile Brush");

        let current_brush = control_state.tile_brush;
        ui.menu_button(current_brush.display_name(), |ui| {
            TileType::all()
                .iter()
                .filter(|t| **t != current_brush)
                .for_each(|t| {
                    if ui.button(t.display_name()).clicked() {
                        control_state.tile_brush = *t;
                    }
                })
        });
    });
}

use raycast::CursorState;
fn paint_tile(
    button: Res<Input<MouseButton>>,
    cursor: Res<CursorState>,
    control_state: ResMut<SandboxControlState>,
    mut map: ResMut<map::Map>,
) {
    if button.pressed(MouseButton::Left) {
        match *cursor {
            CursorState::OnTile(coord) => {
                map.set_tile(coord, control_state.tile_brush);
            }
            _ => {}
        }
    }
}
