use super::*;
use std::collections::HashMap;

pub struct SpeakerMap(HashMap<Speaker, Handle<Image>>);
pub struct ActiveSpeakers(Vec<Entity>);
pub struct SpeakerPlugin;

impl Plugin for SpeakerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveSpeakers(Vec::new()))
            .add_startup_system(initialize_speaker_portraits)
            .add_system(update_and_position_speakers.run_in_state(GameMode::VNMode));
    }
}

fn initialize_speaker_portraits(assets: Res<AssetServer>, mut commands: Commands) {
    use serde_json;
    use serde_json::{Map, Value};
    use std::fs::File;
    use std::io::prelude::*;

    let mut map = HashMap::new();

    let file_path = format!("assets/vn_scenes/characters/character_data.json");

    let mut file = File::open(&file_path).expect("Couldn't open face data json.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json: Map<String, Value> = serde_json::from_str(&contents).expect("Could not parse json");

    json.iter().for_each(|(key, val)| {
        let speaker = Speaker::from_key(key).expect("Invalid Speaker Key");

        match val {
            serde_json::Value::String(s) => {
                let handle = assets.load(&format!("vn_scenes/characters/{s}"));

                map.insert(speaker, handle);
            }
            _ => panic!("Expected strings in face data json."),
        }
    });

    commands.insert_resource(SpeakerMap(map));
}

fn update_and_position_speakers(
    mut active_speakers: ResMut<ActiveSpeakers>,
    current_scene: Res<VNScene>,
    sprites: Res<SpeakerMap>,
    mut commands: Commands,
) {
    while current_scene.speaker_count() > active_speakers.0.len() {
        let new = commands.spawn().id();
        active_speakers.0.push(new);
    }

    while active_speakers.0.len() > current_scene.speaker_count() {
        commands
            .entity(active_speakers.0.pop().unwrap())
            .despawn_recursive();
    }
    let mut count_left = 0;
    let mut count_right = 0;

    current_scene
        .speakers()
        .iter()
        .enumerate()
        .for_each(|(idx, d_speaker)| {
            let mut e_cmd = commands.entity(active_speakers.0[idx]);
            e_cmd.insert(Name::new(d_speaker.speaker.name.to_owned()));

            if let Some(tex_handle) = sprites.0.get(&d_speaker.speaker) {
                e_cmd.insert_bundle(SpriteBundle {
                    texture: tex_handle.clone(),
                    ..default()
                });

                let mut tform = Transform::from_translation(Vec3::Z);

                const SPACE_PER_PORTRAIT: f32 = 200.0;
                match d_speaker.side {
                    Side::Left => {
                        const START_LEFT: f32 = -500.0;
                        tform.translation.x = START_LEFT + (count_left as f32 * SPACE_PER_PORTRAIT);
                        count_left += 1;
                    }
                    Side::Right => {
                        const START_RIGHT: f32 = 500.0;

                        tform = tform.with_scale(Vec3::new(-1.0, 1.0, 1.0));
                        tform.translation.x =
                            START_RIGHT - (count_right as f32 * SPACE_PER_PORTRAIT);
                        count_right += 1;
                    }
                }

                e_cmd.insert(tform);
            }
        });
}
