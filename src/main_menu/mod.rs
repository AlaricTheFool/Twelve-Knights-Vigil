use crate::prelude::*;
use bevy::app::AppExit;

const FONT_SIZE: f32 = 32.0;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameMode::MainMenu, setup_main_menu)
            .add_system(main_menu_ui.run_in_state(GameMode::MainMenu));
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
}

fn main_menu_ui(
    mut egui_context: ResMut<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|mut ui| {
                ui.add_space(96.0);
                ui.heading(
                    egui::RichText::new("12 Knight's Vigil")
                        .size(FONT_SIZE)
                        .color(egui::Color32::BLACK),
                );

                ui.add_space(128.0);
                if menu_button(&mut ui, "Play").clicked() {
                    commands.insert_resource(NextState(GameMode::TDMode));
                }

                if menu_button(&mut ui, "Quit").clicked() {
                    exit.send(AppExit);
                }
            });
        });
}

fn menu_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    ui.button(egui::RichText::new(text).size(FONT_SIZE))
}
