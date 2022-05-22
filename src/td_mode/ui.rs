use super::*;

pub struct TDModeUIPlugin;

impl Plugin for TDModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_status_panel.run_in_state(GameMode::TDMode));
    }
}

fn render_status_panel(
    mut egui_context: ResMut<EguiContext>,
    lives: Res<life::Lives>,
    gold: Res<gold::Gold>,
) {
    egui::TopBottomPanel::top("Status").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("<3: {}", lives.get()));
            ui.add_space(64.0);
            ui.label(format!("Gold: {}", gold.get()));
        });
    });
}
