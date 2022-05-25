use super::*;

pub struct TDModeUIPlugin;

impl Plugin for TDModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_status_panel.run_in_state(GameMode::TDMode))
            .add_system(draw_tower_build_ui.run_in_state(GameMode::TDMode));
    }
}

fn render_status_panel(
    mut egui_context: ResMut<EguiContext>,
    lives: Res<life::Lives>,
    gold: Res<gold::Gold>,
    ui_action: Res<UIAction>,
) {
    egui::TopBottomPanel::top("Status").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("<3: {}", lives.get()));
            ui.add_space(64.0);
            ui.label(format!("Gold: {}", gold.get()));

            ui.add_space(256.0);

            #[cfg(feature = "debug")]
            {
                ui.label("Debug Mode");
                ui.label(format!("UIAction: {:?}", *ui_action));
            }
        });
    });
}

fn draw_tower_build_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_action: ResMut<UIAction>,
    k_statuses: Res<KnightStatuses>,
) {
    egui::TopBottomPanel::bottom("Tower Building").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            [TowerType::Short, TowerType::Medium, TowerType::Tall]
                .iter()
                .for_each(|t_type| {
                    let txt = match *t_type {
                        TowerType::Short => "Short Tower",
                        TowerType::Medium => "Medium Tower",
                        TowerType::Tall => "Tall Tower",
                    };

                    if ui.button(txt).is_pointer_button_down_on() && *ui_action == UIAction::None {
                        *ui_action = UIAction::BuildTower(t_type.clone());
                    }
                });

            ui.add_space(128.0);

            k_statuses.0.iter().for_each(|(knight, status)| {
                let button = egui::Button::new(knight.get_name());

                if ui
                    .add_enabled(*status == KUsageStatus::Ready, button)
                    .is_pointer_button_down_on()
                    && *ui_action == UIAction::None
                {
                    *ui_action = UIAction::PlaceKnight(knight.clone());
                }
            });
        });
    });
}
