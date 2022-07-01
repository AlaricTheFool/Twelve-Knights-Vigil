//! Code for the map editor/sandbox tools

use super::td_mode_prelude::*;
use super::{elements::ApplyElementMessage, elements::ElementalAffliction, *};
use bevy_egui::{egui, EguiContext};
use map::{MapRoot, TileType};

const FIXED_STEP_MS: u64 = 20;
const APPLICATOR_ELEMENTS_PER_SECOND: u32 = 10;
const APPLICATOR_ELEMENTS_PER_FRAME: f32 =
    (APPLICATOR_ELEMENTS_PER_SECOND as f32) / (1000 / FIXED_STEP_MS) as f32;

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SandboxControlState::new())
            .add_system(sandbox_ui.run_in_state(GameState::TDMode))
            .add_system(tile_inspector_ui.run_in_state(GameState::TDMode));

        let mut fixed_stage = SystemStage::parallel();
        fixed_stage.add_system(use_tool.run_in_state(GameState::TDMode));

        app.add_stage_before(
            CoreStage::Update,
            "tool_fixed_stage",
            FixedTimestepStage::new(Duration::from_millis(FIXED_STEP_MS)).with_stage(fixed_stage),
        );
    }
}

struct SandboxControlState {
    new_dimensions: (usize, usize),
    current_tool: Tool,
    selected_tile: Option<Entity>,
}

impl SandboxControlState {
    fn new() -> Self {
        Self {
            new_dimensions: (8, 8),
            current_tool: Tool::Select,
            selected_tile: None,
        }
    }
}

#[derive(Debug)]
enum Tool {
    Select,
    TileBrush(TileType),
    ElementApplicator(Element, f32),
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

        ui.heading("Tools");

        if ui.button("Select").clicked() {
            control_state.current_tool = Tool::Select;
        }

        let t_brush_text = if let Tool::TileBrush(tile) = control_state.current_tool {
            format!("{tile}")
        } else {
            format!("Tile Brush")
        };
        ui.menu_button(t_brush_text, |ui| {
            TileType::all().iter().for_each(|t| {
                if ui.button(format!("{t}")).clicked() {
                    control_state.current_tool = Tool::TileBrush(*t);
                }
            })
        });

        let e_brush_text = if let Tool::ElementApplicator(element, _) = control_state.current_tool {
            format!("{element}")
        } else {
            format!("Element Applicator")
        };
        ui.menu_button(e_brush_text, |ui| {
            Element::all().iter().for_each(|e| {
                if ui.button(format!("{e}")).clicked() {
                    control_state.current_tool = Tool::ElementApplicator(*e, 0.0);
                }
            })
        });
    });
}

fn tile_inspector_ui(
    tile_query: Query<(&TileType, &Coordinate, Option<&ElementalAffliction>)>,
    control_state: Res<SandboxControlState>,
    mut egui_context: ResMut<EguiContext>,
) {
    if let Some(tile_entity) = control_state.selected_tile {
        if let Ok((tile_type, coord, elements)) = tile_query.get(tile_entity) {
            egui::Window::new("Tile Inspector").show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Coordinates: {coord}"));
                ui.label(format!("Tile Type: {tile_type}"));

                if let Some(applied_elements) = elements {
                    ui.label("Applied Elements:");
                    ui.label(format!("{applied_elements}"));
                }
            });
        }
    }
}

use raycast::CursorState;
fn use_tool(
    button: Res<Input<MouseButton>>,
    cursor: Res<CursorState>,
    map_root_query: Query<&MapRoot>,
    mut control_state: ResMut<SandboxControlState>,
    mut map: ResMut<map::Map>,
    mut commands: Commands,
) {
    if button.pressed(MouseButton::Left) {
        match *cursor {
            CursorState::OnTile(tile_entity, coord) => match control_state.current_tool {
                Tool::Select => {
                    if button.just_pressed(MouseButton::Left) {
                        control_state.selected_tile = Some(tile_entity);
                    }
                }

                Tool::TileBrush(tile_type) => {
                    let prev_tile_type = map.tile_type_at_coord(coord).unwrap();

                    if *prev_tile_type != tile_type {
                        let map_root = map_root_query.get_single().unwrap();
                        let tile_idx = map.coord_to_idx(coord);
                        let tile_entity = map_root.tile_entities[tile_idx];
                        match tile_type {
                            TileType::Fire => {
                                commands
                                    .spawn()
                                    .insert(Message)
                                    .insert(Target(tile_entity))
                                    .insert(ApplyElement)
                                    .insert(ElementalAffliction::single(Element::Fire, 100));
                            }
                            _ => {}
                        }
                    }

                    map.set_tile(coord, tile_type);
                }

                Tool::ElementApplicator(element, mut accumulation) => {
                    accumulation += APPLICATOR_ELEMENTS_PER_FRAME;
                    let mut stacks_to_apply_this_frame = 0;

                    while accumulation > 1.0 {
                        stacks_to_apply_this_frame += 1;
                        accumulation -= 1.0;
                    }

                    if stacks_to_apply_this_frame > 0 {
                        commands.spawn_bundle(ApplyElementMessage::single_element(
                            element,
                            stacks_to_apply_this_frame,
                            tile_entity,
                        ));
                    }

                    control_state.current_tool = Tool::ElementApplicator(element, accumulation);
                } //_ => warn!("Did not implement tool: {:?}", control_state.current_tool),
            },
            _ => {}
        }
    }
}
