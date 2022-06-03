use crate::prelude::*;

mod parser;
mod scene;

pub use parser::*;
pub use scene::*;

pub struct VNModePlugin;

impl Plugin for VNModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VNScene::new())
            .add_enter_system(GameMode::VNMode, load_test_scene)
            .add_system(render_scene.run_in_state(GameMode::VNMode));
    }
}

fn load_test_scene(mut commands: Commands) {
    let scene = load_scene("test_scenes/test_dialogue").unwrap();
    commands.insert_resource(VNScene::from_events(scene));
}

fn render_scene(mut scene: ResMut<VNScene>, mut egui_context: ResMut<EguiContext>) {
    if let Some(event) = scene.current() {
        match event {
            VNEvent::Dialogue(speaker, line) => {
                egui::TopBottomPanel::bottom("Dialogue").show(egui_context.ctx_mut(), |ui| {
                    ui.label(egui::RichText::new(speaker.name).size(20.0));
                    ui.separator();
                    ui.label(egui::RichText::new(line).size(15.0));

                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        if ui.button("Next").clicked() {
                            scene.next();
                        }
                    });
                });
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VNEvent {
    Dialogue(Speaker, String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Speaker {
    name: String,
}

impl Speaker {
    pub fn from_knight(knight: Knight) -> Self {
        Self {
            name: knight.get_name().to_string(),
        }
    }

    pub fn player() -> Self {
        Self {
            name: "Me".to_string(),
        }
    }

    pub fn unknown() -> Self {
        Self {
            name: "???".to_string(),
        }
    }

    pub fn narrator() -> Self {
        Self {
            name: "Narrator".to_string(),
        }
    }

    pub fn from_key(key: &str) -> Result<Self, String> {
        match key {
            "PLAYER" => Ok(Self::player()),
            "NARRATOR" => Ok(Self::narrator()),
            "AVERAGE_KNIGHT" => Ok(Self::from_knight(Knight::Normal)),
            _ => Err("Unrecognized Character Code: {key}".to_string()),
        }
    }
}
