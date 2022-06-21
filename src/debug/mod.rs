use crate::prelude::*;

mod fps;

/*
const VERTICAL_MARKER_HEIGHT: f32 = 1.0;

pub struct DebugModels {
    pub vertical_marker_mesh: Handle<Mesh>,
    pub debug_material: Handle<StandardMaterial>,
}
*/

pub struct TKDebugPlugin;

impl Plugin for TKDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(fps::FPSTrackerPlugin)
            //   .add_startup_system(initialize_debug_models)
            .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
        //            .add_system(add_tilemap_point_markers);
    }
}

/*
fn initialize_debug_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut debug_material: StandardMaterial = Color::rgba(0.7, 0.3, 0.3, 0.2).into();
    debug_material.alpha_mode = bevy::pbr::AlphaMode::Blend;
    commands.insert_resource(DebugModels {
        vertical_marker_mesh: meshes.add(Mesh::from(shape::Box::new(
            0.1,
            VERTICAL_MARKER_HEIGHT,
            0.1,
        ))),
        debug_material: materials.add(debug_material),
    });
}

pub fn add_tilemap_point_markers(
    mut commands: Commands,
    mut query: Query<(Entity, &Track), Added<Track>>,
    models: Res<DebugModels>,
) {
    for (entity, track) in query.iter() {
        commands
            .spawn()
            .insert(Parent(entity))
            .insert(Name::new("Path Points"))
            .insert_bundle(TransformBundle::identity())
            .with_children(|p| {
                track.points.iter().enumerate().for_each(|(idx, point)| {
                    p.spawn()
                        .insert(Name::new(format!("Point #{idx}")))
                        .insert_bundle(TransformBundle::from_transform(*point))
                        .with_children(|p| {
                            p.spawn_bundle(PbrBundle {
                                mesh: models.vertical_marker_mesh.clone(),
                                material: models.debug_material.clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    0.0,
                                    VERTICAL_MARKER_HEIGHT * 0.5,
                                    0.0,
                                )),
                                ..default()
                            });
                        });
                });
            });
    }
}
*/
