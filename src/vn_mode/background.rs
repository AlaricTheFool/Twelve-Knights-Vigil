use super::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub struct BGMap(HashMap<String, Handle<Image>>);
pub struct CurrentBG(Handle<Image>);
pub struct NextBG(pub String);

#[derive(Component)]
pub struct Background;

pub fn initialize_bg_asset_map(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut file = File::open("assets/vn_scenes/backgrounds/bg_list.json")
        .expect("Failed to open bg_list file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents of bg list file.");

    let v: serde_json::Value =
        serde_json::from_str(&contents).expect("Failed to parse JSON in bg_list");

    match v {
        serde_json::Value::Object(map) => {
            let mut h_map = HashMap::new();

            map.iter()
                .enumerate()
                .for_each(|(idx, (key, val))| match val.clone() {
                    serde_json::Value::String(s) => {
                        let handle = asset_server.load(&format!("vn_scenes/backgrounds/{s}"));
                        h_map.insert(key.to_owned(), handle.clone());

                        if idx == 0 {
                            commands.insert_resource(CurrentBG(handle.clone()));
                            commands.insert_resource(NextBG("".to_string()));
                        }
                    }
                    _ => {
                        panic!("Incorrectly defined background key {key}");
                    }
                });

            info!("{h_map:?}");
            commands.insert_resource(BGMap(h_map));
        }

        _ => {
            panic!("The BG Map file was parsed as something other than a json object.");
        }
    }
}

pub fn initialize_background(mut commands: Commands) {
    commands
        .spawn()
        .insert(Background)
        .insert_bundle(SpriteBundle::default());
}

pub fn update_background(
    mut bg_query: Query<(&mut Handle<Image>, &mut Sprite), With<Background>>,
    current_bg: Res<CurrentBG>,
) {
    bg_query.iter_mut().for_each(|(mut img, mut sprite)| {
        if *img != current_bg.0 {
            *img = current_bg.0.clone();
        }

        sprite.custom_size = Some(Vec2::new(1280.0, 720.0));
    });
}

pub fn switch_backgrounds(
    mut next_bg: ResMut<NextBG>,
    mut current_bg: ResMut<CurrentBG>,
    bg_map: Res<BGMap>,
) {
    if next_bg.0 != "" {
        if let Some(new_bg_handle) = bg_map.0.get(&next_bg.0) {
            current_bg.0 = new_bg_handle.clone();
        } else {
            error!(
                "Attempted to switch to an invalid background: \"{}\"",
                next_bg.0
            );
        }
        next_bg.0 = "".to_string();
    }
}
