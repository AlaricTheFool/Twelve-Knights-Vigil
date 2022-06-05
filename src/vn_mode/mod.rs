use crate::prelude::*;

mod background;
mod parser;
mod scene;

pub use background::*;
pub use parser::*;
pub use scene::*;

pub struct VNModePlugin;

impl Plugin for VNModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VNScene::new())
            .add_startup_system(initialize_bg_asset_map)
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
        let frame = egui::Frame::dark_canvas(&egui::Style::default());
        match event {
            VNEvent::Dialogue(speaker, line) => {
                egui::Window::new("Dialogue")
                    .title_bar(false)
                    .auto_sized()
                    .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0.0, -8.0))
                    .frame(frame)
                    .show(egui_context.ctx_mut(), |ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(speaker.name).size(40.0));
                            ui.separator();
                            ui.label(egui::RichText::new(line).size(30.0));

                            ui.add_space(30.0);

                            if ui.button(egui::RichText::new("Next").size(15.0)).clicked() {
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
