mod components;
mod enemy;
mod messages;
mod tilemap;

#[cfg(feature = "debug")]
mod debug;

mod prelude {
    pub use crate::components::*;
    pub use crate::enemy::*;
    pub use crate::messages::*;
    pub use crate::tilemap::*;
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use rand::*;

    #[cfg(feature = "debug")]
    pub use debug;
}

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Twelve Knight's Vigil".to_string(),
        width: 1280.0,
        height: 720.0,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .insert_resource(CurrentMap(None))
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_startup_system(respawn_tilemap)
    .add_system_to_stage(CoreStage::PreUpdate, respawn_tilemap.run_if(respawn_pushed))
    .add_system(initialize_tilemap)
    .add_system(rotate_tiles)
    .add_system(set_light_direction)
    .add_system(spawn_enemies)
    .add_system(move_track_followers)
    .add_system(update_track_followers);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(debug::TKDebugPlugin);
    }

    app.run();
}

fn respawn_pushed(input: Res<Input<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::R)
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let tile_models = TileModels {
        empty: assets.load("models/tile.glb#Scene0"),
        rock: assets.load("models/tile_rock.glb#Scene0"),
        straight: assets.load("models/tile_straight.glb#Scene0"),
        corner: assets.load("models/tile_cornerSquare.glb#Scene0"),
        tree: assets.load("models/tile_tree.glb#Scene0"),
    };

    commands.insert_resource(tile_models);

    let enemy_models = EnemyModels {
        basic: assets.load("models/enemy_ufoRed.glb#Scene0"),
    };

    commands.insert_resource(enemy_models);

    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 8.0, 16.0)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
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
        ..default()
    });
}

fn respawn_tilemap(
    mut commands: Commands,
    query: Query<Entity, With<TileMap>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut current_map: ResMut<CurrentMap>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let new_map = commands
        .spawn_bundle(TransformBundle::from(Transform { ..default() }))
        .insert(TileMap::new(16, 16))
        .insert(Name::new("Map"))
        .id();

    current_map.0 = Some(new_map);
}

fn rotate_tiles(time: Res<Time>, mut query: Query<&mut Transform, With<TileMap>>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 20.0,
            0.0,
        )
    }
}

fn initialize_tilemap(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileMap), Added<TileMap>>,
    models: Res<TileModels>,
) {
    for (entity, mut map) in query.iter_mut() {
        map.initialize_tiles(entity, &mut commands, &models);
    }
}

fn set_light_direction(mut query: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            -std::f32::consts::FRAC_PI_4,
            0.0,
            -std::f32::consts::FRAC_PI_4,
        )
    }
}
