use super::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub struct BGMap(HashMap<String, Handle<Image>>);

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

            map.iter().for_each(|(key, val)| match val.clone() {
                serde_json::Value::String(s) => {
                    h_map.insert(
                        key.to_owned(),
                        asset_server.load(&format!("vn_scenes/backgrounds/{s}")),
                    );
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
