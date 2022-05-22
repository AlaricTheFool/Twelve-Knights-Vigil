use crate::prelude::*;
use crate::td_mode::tower_building::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState::NoTarget)
            .insert_resource(MapControl::new())
            .add_system(update_map_rotation_dir.run_in_state(GameMode::TDMode))
            .add_system(send_build_tower_messages.run_in_state(GameMode::TDMode));
    }
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
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    match *cursor_state {
        CursorState::OnTile(coord) => {
            if mouse_btn.just_pressed(MouseButton::Left) && current_map.0.is_some() {
                commands
                    .spawn()
                    .insert(Message)
                    .insert(BuildTower { location: coord })
                    .insert(Target(current_map.0.unwrap()));
            }
        }
        _ => {}
    }
}
