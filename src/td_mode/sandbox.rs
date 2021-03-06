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
            .add_system(place_debug_cubes_along_path.run_in_state(GameState::TDMode))
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
    redraw_path: bool,
}

impl SandboxControlState {
    fn new() -> Self {
        Self {
            new_dimensions: (8, 8),
            current_tool: Tool::Select,
            selected_tile: None,
            redraw_path: true,
        }
    }
}

#[derive(Debug)]
enum Tool {
    Select,
    TileBrush(TileType),
    StructureBrush(Structure),
    ElementApplicator(Element, f32),
    PlacePiece(TilePiece),
}

/// A temporary enum? Eventually this will include towers, and any other things that go on top of
/// terrain.
#[derive(Debug, Copy, Clone)]
enum TilePiece {
    PathStart,
    PathEnd,
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

        ui.horizontal(|ui| {
            if ui.button("Start").clicked() {
                control_state.current_tool = Tool::PlacePiece(TilePiece::PathStart);
            }
            if ui.button("End").clicked() {
                control_state.current_tool = Tool::PlacePiece(TilePiece::PathEnd);
            }
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
                        map.set_tile(coord, Some(tile_type), None);
                    }
                }

                Tool::StructureBrush(structure) => {
                    let prev_structure = map.structure_at_coord(coord).unwrap();

                    if *prev_structure != structure {
                        map.set_tile(coord, None, Some(structure));
                    }
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
                }

                Tool::PlacePiece(tile_piece) => {
                    if button.just_pressed(MouseButton::Left) {
                        control_state.redraw_path = true;
                        match tile_piece {
                            TilePiece::PathStart => {
                                map.wave_entry_coord = coord;
                            }

                            TilePiece::PathEnd => {
                                map.wave_exit_coord = coord;
                            }
                        }
                    }
                } //_ => warn!("Did not implement tool: {:?}", control_state.current_tool),
            },
            _ => {}
        }
    }
}

/// A tag struct for the astar path markers temporarily being displayed.
#[derive(Component)]
struct DebugPoint;

fn place_debug_cubes_along_path(
    debug_obj_query: Query<Entity, With<DebugPoint>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
    mut control_state: ResMut<SandboxControlState>,
    mut commands: Commands,
) {
    if control_state.redraw_path || map.is_changed() {
        let start_coord = map.wave_entry_coord;
        let end_coord = map.wave_exit_coord;
        let result = astar(
            &start_coord,
            |p| map.find_astar_successors(*p),
            |p| p.distance(&end_coord),
            |p| *p == end_coord,
        );

        if let Some(path) = result {
            debug_obj_query.iter().for_each(|e| {
                commands.entity(e).despawn_recursive();
            });

            path.0.iter().for_each(|coord| {
                let tlation = (*coord * Vec3::new(1.0, 0.0, 1.0))
                    + Vec3::new(
                        map.dimensions.0 as f32 * -0.5,
                        0.0,
                        map.dimensions.1 as f32 * -0.5,
                    );
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: Transform::from_translation(tlation),
                        ..default()
                    })
                    .insert(DebugPoint);
            });
        }

        control_state.redraw_path = false;
    }
}
