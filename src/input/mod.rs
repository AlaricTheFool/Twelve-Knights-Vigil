use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapControl::new())
            .add_system(update_map_rotation_dir);
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

fn update_map_rotation_dir(keys: Res<Input<KeyCode>>, mut map_control: ResMut<MapControl>) {
    let (left_pressed, right_pressed) = (keys.pressed(KeyCode::Q), keys.pressed(KeyCode::E));

    map_control.rotation_dir = match (left_pressed, right_pressed) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };
}
