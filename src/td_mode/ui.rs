use super::*;

pub struct TDModeUIPlugin;

impl Plugin for TDModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_status_panel.run_in_state(GameMode::TDMode))
            .add_system(draw_tower_inspector_ui.run_in_state(GameMode::TDMode))
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

fn draw_tower_inspector_ui(
    mut egui_context: ResMut<EguiContext>,
    selection: Res<SelectionTarget>,
    tower_query: Query<(&Tower, &TowerType)>,
    knight_query: Query<&Knight>,
    power_bar_query: Query<&PowerBar>,
    tic_tac_toe_query: Query<&tic_tac_toe::TicTacToe>,
    damage_query: Query<&Damage>,
    cd_query: Query<&Cooldown>,
    mut commands: Commands,
) {
    if let Some(selected_entity) = selection.0 {
        if let Ok((_, tower_type)) = tower_query.get(selected_entity) {
            let tower_name = if let Ok(knight) = knight_query.get(selected_entity) {
                format!("{}'s {tower_type:?} Tower", knight.get_name())
            } else {
                format!("{tower_type:?} Tower")
            };
            egui::Window::new(tower_name).show(egui_context.ctx_mut(), |ui| {
                if let Ok(power_bar) = power_bar_query.get(selected_entity) {
                    ui.horizontal(|ui| {
                        let pct = power_bar.get_pct();
                        ui.label(format!("{}%", power_bar.get_actual()));
                        ui.add(egui::widgets::ProgressBar::new(pct));
                    });
                }

                if let Ok(tic_tac_toe) = tic_tac_toe_query.get(selected_entity) {
                    if let Some(played_move_board) = tic_tac_toe.render_egui(ui) {
                        commands.entity(selected_entity).insert(played_move_board);
                    }
                }

                ui.horizontal(|ui| {
                    if let Ok(damage) = damage_query.get(selected_entity) {
                        ui.label(format!("Damage: {}", damage.0));
                    }

                    if let Ok(cd) = cd_query.get(selected_entity) {
                        ui.label(format!("Shots per Second: {:.2}", cd.shots_per_second()));
                    }
                });

                if ui.button("Sell").clicked() {
                    commands
                        .spawn()
                        .insert(Message)
                        .insert(Sell)
                        .insert(Target(selected_entity));
                }
            });
        }
    }
}
