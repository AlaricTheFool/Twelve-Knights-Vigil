//! Functionality for the camera and its movement in TD Maps

use crate::prelude::*;

/// Units to zoom the camera in or out for each click of the mouse wheel
const ZOOM_RATE: f32 = 1.0;

/// Tag component for the camera's pivot point
#[derive(Component)]
struct CameraArm;

/// Tag Component for the player-controlled camera
#[derive(Component)]
struct PlayerCam;

/// Resource with the vars used to manipulate the camera
struct CameraControl {
    /// +1.0 = Clockwise, -1.0 = Counter-Clockwise
    rotation_dir: f32,
    /// Horizontal movement on the flat plane of the map.
    move_dir: Vec2,
    /// +1.0 = Zoom in, -1.0 = Zoom out
    /// Stored in whole units of mouse wheel rotations rather than a direction
    zoom_dir: f32,
}

impl CameraControl {
    fn zero() -> Self {
        Self {
            rotation_dir: 0.0,
            move_dir: Vec2::ZERO,
            zoom_dir: 0.0,
        }
    }
}

pub struct TDCameraPlugin;

impl Plugin for TDCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::TDMode, setup)
            .insert_resource(CameraControl::zero())
            .add_system(zoom_camera.run_in_state(GameState::TDMode))
            .add_system(update_camera_controls.run_in_state(GameState::TDMode));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::identity())
        .insert(CameraArm)
        .insert(Name::new("Player Camera"))
        .with_children(|p| {
            p.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            })
            .insert(PlayerCam);
        });
}

/// Update the camera's control struct based on user input.
///
/// NOTE: This may be moved to a more central input module at some point when keybinding and
/// gamepad support are added.
fn update_camera_controls(
    mut controls: ResMut<CameraControl>,
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
) {
    // Camera movement
    let (m_up, m_down, m_right, m_left) = (
        keys.pressed(KeyCode::W),
        keys.pressed(KeyCode::S),
        keys.pressed(KeyCode::D),
        keys.pressed(KeyCode::A),
    );

    let camera_move_dir = Vec2::new(bools_to_axis(m_up, m_down), bools_to_axis(m_right, m_left));

    controls.move_dir = camera_move_dir;

    // Camera Rotation
    let (r_clock, r_counterclock) = (keys.pressed(KeyCode::E), keys.pressed(KeyCode::Q));
    let cam_rotation_dir = bools_to_axis(r_clock, r_counterclock);

    controls.rotation_dir = cam_rotation_dir;

    // Camera Zoom
    use bevy::input::mouse::MouseScrollUnit;

    let cam_zoom = scroll_evr.iter().fold(0.0, |acc, ev| match ev.unit {
        MouseScrollUnit::Line => acc + ev.y,

        _ => acc,
    });

    // We store zoom in number of mouse wheel rotations.
    controls.zoom_dir += cam_zoom;
}

/// Zoom the camera in or out based on the controls.
/// This is based on the magnitude of the zoom value in the controls rather than
/// being a constant direction so it doesn't need to be frame limited.
fn zoom_camera(
    mut tform_query: Query<&mut Transform, With<PlayerCam>>,
    mut controls: ResMut<CameraControl>,
) {
    if controls.zoom_dir != 0.0 {
        let mut tform = tform_query.single_mut();
        *tform.translation =
            (tform.translation + tform.forward() * controls.zoom_dir * ZOOM_RATE).into();

        controls.zoom_dir = 0.0;
    }
}

/// Convert two pools indicating direction to a float based on which is pressed.
///
/// Used for converting inputs to multipliers.
fn bools_to_axis(positive: bool, negative: bool) -> f32 {
    match (positive, negative) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    }
}
