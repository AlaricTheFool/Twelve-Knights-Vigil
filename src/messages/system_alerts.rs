use super::*;
use crate::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

const ALERT_TIME_SECS: f32 = 1.0;
const ALERT_TIME_MS: u64 = ALERT_TIME_SECS as u64 * 1000;

const MAX_DISPLAYED_ALERTS: usize = 8;

pub struct SystemAlerts(VecDeque<(String, Duration)>);

impl SystemAlerts {
    pub fn new() -> Self {
        SystemAlerts(VecDeque::with_capacity(MAX_DISPLAYED_ALERTS))
    }
}

pub fn create_system_alert_message(alerts: &mut SystemAlerts, message: &str) {
    alerts
        .0
        .push_back((message.to_string(), Duration::from_millis(ALERT_TIME_MS)));

    if alerts.0.len() > MAX_DISPLAYED_ALERTS {
        alerts.0.pop_front();
    };
}

pub fn display_system_messages(
    mut egui_context: ResMut<EguiContext>,
    time: Res<Time>,
    mut alerts: ResMut<SystemAlerts>,
) {
    if alerts.0.len() > 0 {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(egui_context.ctx_mut(), |ui| {
                ui.push_id("Alerts", |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(64.0);

                        //TODO: Sort these by remaining duration so they remain in a consistent order.
                        alerts.0 = alerts
                            .0
                            .iter()
                            .map(|(alert, mut timer)| {
                                timer = timer.saturating_sub(time.delta());

                                (alert.to_owned(), timer)
                            })
                            .filter(|(_, duration)| *duration != Duration::ZERO)
                            .collect();

                        alerts.0.iter().rev().for_each(|(msg, duration)| {
                            let pct = duration.as_millis() as f32 / ALERT_TIME_MS as f32;

                            let color = egui::Color32::from_rgba_unmultiplied(
                                255,
                                0,
                                0,
                                (255.0 * pct).floor().max(0.0) as u8,
                            );
                            ui.colored_label(color, egui::RichText::new(msg).size(20.0));
                        });
                    });
                });
            });
    }
}
