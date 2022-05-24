use crate::prelude::*;

//TODO: This shouldn't need to be public.
pub mod enemy;
pub mod gold;
pub mod raycast;
pub mod tower_building;

mod life;
mod ui;

use enemy::*;
use raycast::*;
use tower_building::*;

pub struct TDModePlugin;

impl Plugin for TDModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.4, true)))
            .insert_resource(CurrentMap(None))
            .add_plugin(PickablePlugin)
            .add_plugin(TowerPlugin)
            .add_plugin(KnightPlugin)
            .add_plugin(life::LifePlugin)
            .add_plugin(gold::GoldPlugin)
            .add_plugin(ui::TDModeUIPlugin)
            .add_enter_system(GameMode::TDMode, load_td_models)
            .add_enter_system(GameMode::TDMode, spawn_tdmode_resources)
            .add_enter_system(GameMode::TDMode, spawn_camera_and_lighting)
            .add_enter_system(GameMode::TDMode, respawn_tilemap)
            .add_exit_system(GameMode::TDMode, destroy_everything)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                respawn_tilemap
                    .run_in_state(GameMode::TDMode)
                    .run_if(respawn_pushed),
            )
            .add_system(spawn_enemies.run_in_state(GameMode::TDMode))
            .add_system(update_track_followers.run_in_state(GameMode::TDMode))
            .add_system(initialize_tilemap.run_in_state(GameMode::TDMode))
            .add_system(handle_build_tower_messages.run_in_state(GameMode::TDMode))
            .add_system(handle_loss.run_in_state(GameMode::TDMode))
            .add_system(
                return_to_menu
                    .run_in_state(GameMode::TDMode)
                    .run_if(menu_button_pressed),
            );

        let mut fixed_stage = SystemStage::parallel();
        fixed_stage
            .add_system(move_track_followers)
            .add_system(rotate_tiles);

        app.add_stage_before(
            CoreStage::Update,
            "fixed_stages",
            FixedTimestepStage::new(std::time::Duration::from_millis(16)).with_stage(fixed_stage),
        );
    }
}

fn respawn_pushed(input: Res<Input<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::R)
}

fn return_to_menu(mut commands: Commands) {
    commands.insert_resource(NextState(GameMode::MainMenu));
}

fn menu_button_pressed(input: Res<Input<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::P)
}

fn load_td_models(assets: Res<AssetServer>, mut commands: Commands) {
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
}

fn spawn_camera_and_lighting(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.7, 8.0, 16.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        })
        .insert(RayCastSource::<PickableRaycastSet>::new());
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
        transform: Transform::identity().with_rotation(Quat::from_euler(
            EulerRot::ZYX,
            -std::f32::consts::FRAC_PI_4,
            0.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        ..default()
    });
}

fn spawn_tdmode_resources(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
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
    life::send_reset_lives_message(&mut commands);
    gold::send_reset_gold_message(&mut commands);
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

fn rotate_tiles(map_control: Res<MapControl>, mut query: Query<&mut Transform, With<TileMap>>) {
    for mut transform in query.iter_mut() {
        let added_rot = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            -map_control.rotation_dir * std::f32::consts::PI * 2.0 * 0.25 * (1.0 / 60.0),
            0.0,
        );
        transform.rotate(added_rot);
    }
}

fn handle_loss(
    game_over_query: Query<(Entity, &Message, &life::OutOfLives)>,
    mut commands: Commands,
) {
    game_over_query.iter().for_each(|(e, _, _)| {
        commands.insert_resource(NextState(GameMode::MainMenu));
        commands.entity(e).insert(IsHandled);
    });
}

// This is a little inelegant but it serves purposes for now.
fn destroy_everything(query: Query<Entity>, mut commands: Commands) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
