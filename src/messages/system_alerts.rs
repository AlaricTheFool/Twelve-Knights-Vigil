use super::*;
use crate::prelude::*;
use std::time::Duration;

const ALERT_TIME_SECS: f32 = 1.0;
const ALERT_TIME_MS: u64 = ALERT_TIME_SECS as u64 * 1000;

#[derive(Component)]
pub struct SystemAlert(pub String);

#[derive(Component)]
pub struct MessageTimer(Duration);

pub fn create_system_alert_message(commands: &mut Commands, message: &str) {
    commands
        .spawn()
        .insert(Message)
        .insert(MessageTimer(Duration::from_millis(ALERT_TIME_MS)))
        .insert(SystemAlert(message.to_string()));
}

pub fn display_system_messages(
    mut egui_context: ResMut<EguiContext>,
    mut display_query: Query<(Entity, &SystemAlert, &mut MessageTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(64.0);

                //TODO: Sort these by remaining duration so they remain in a consistent order.
                display_query
                    .iter_mut()
                    .for_each(|(entity, alert, mut timer)| {
                        if let Some(new_dur) = timer.0.checked_sub(time.delta()) {
                            timer.0 = new_dur;
                            let pct = timer.0.as_millis() as f32 / ALERT_TIME_MS as f32;

                            let color = egui::Color32::from_rgba_unmultiplied(
                                255,
                                0,
                                0,
                                (255.0 * pct).floor().max(0.0) as u8,
                            );
                            ui.colored_label(color, egui::RichText::new(&alert.0).size(20.0));
                        } else {
                            commands.entity(entity).insert(IsHandled);
                        }
                    });
            });
        });
}
