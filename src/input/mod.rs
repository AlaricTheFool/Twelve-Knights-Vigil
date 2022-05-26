use bevy::input::keyboard::KeyboardInput;

use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState::NoTarget)
            .insert_resource(UIAction::None)
            .insert_resource(MapControl::new())
            .add_system(update_map_rotation_dir.run_in_state(GameMode::TDMode))
            .add_system(send_reset_message.run_in_state(GameMode::TDMode))
            .add_system_to_stage(CoreStage::Last, reset_ui_action)
            .add_system(send_build_tower_messages.run_in_state(GameMode::TDMode));
    }
}

#[derive(Debug, PartialEq)]
pub enum UIAction {
    None,
    BuildTower(TowerType),
    PlaceKnight(Knight),
}

pub struct MapControl {
    pub rotation_dir: f32,
    //TODO: Panning
}

impl MapControl {
    fn new() -> Self {
        Self { rotation_dir: 0.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorState {
    NoTarget,
    OnTile(Coordinate),
}

fn update_map_rotation_dir(keys: Res<Input<KeyCode>>, mut map_control: ResMut<MapControl>) {
    let (left_pressed, right_pressed) = (keys.pressed(KeyCode::Q), keys.pressed(KeyCode::E));

    map_control.rotation_dir = match (left_pressed, right_pressed) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };
}

fn send_build_tower_messages(
    mouse_btn: Res<Input<MouseButton>>,
    cursor_state: Res<CursorState>,
    ui_action: Res<UIAction>,
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    match *cursor_state {
        CursorState::OnTile(coord) => match *ui_action {
            UIAction::BuildTower(t_type) => {
                if mouse_btn.just_released(MouseButton::Left) && current_map.0.is_some() {
                    commands
                        .spawn()
                        .insert(Message)
                        //TODO: This maybe doesn't belong imported here.
                        .insert(BuildTower {
                            location: coord,
                            t_type,
                        })
                        .insert(Target(current_map.0.unwrap()));
                }
            }

            UIAction::PlaceKnight(k_type) => {
                if mouse_btn.just_released(MouseButton::Left) && current_map.0.is_some() {
                    commands
                        .spawn()
                        .insert(Message)
                        .insert(PlaceKnight {
                            location: coord,
                            knight: k_type,
                        })
                        .insert(Target(current_map.0.unwrap()));
                }
            }

            _ => {}
        },
        _ => {}
    }
}

fn reset_ui_action(mut ui_action: ResMut<UIAction>, mouse_btn: Res<Input<MouseButton>>) {
    match *ui_action {
        UIAction::BuildTower(_) | UIAction::PlaceKnight(_) => {
            if !mouse_btn.pressed(MouseButton::Left) {
                *ui_action = UIAction::None
            }
        }

        _ => {}
    }
}

fn send_reset_message(key_btn: Res<Input<KeyCode>>, mut commands: Commands) {
    if key_btn.just_pressed(KeyCode::R) {
        commands.spawn().insert(Message).insert(Reset);
    }
}
