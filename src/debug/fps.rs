use crate::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::collections::VecDeque;

const FRAME_BUFFER_SIZE: usize = 30;

pub struct FrameTimeBuffer {
    frame_times: VecDeque<f32>,
}

impl FrameTimeBuffer {
    fn new(size: usize) -> Self {
        Self {
            frame_times: VecDeque::from(vec![0.0; size]),
        }
    }

    fn record_frame(&mut self, ms: f32) {
        self.frame_times.push_back(ms);
        self.frame_times.pop_front();
    }

    fn average_seconds(&self) -> f32 {
        self.frame_times.iter().fold(0.0, |accum, ms| accum + ms) / self.frame_times.len() as f32
    }

    fn average_ms(&self) -> f32 {
        1000.0 * self.average_seconds()
    }

    fn average_fps(&self) -> f32 {
        1.0 / self.average_seconds()
    }
}

pub struct FPSTrackerPlugin;

impl Plugin for FPSTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FrameTimeBuffer::new(FRAME_BUFFER_SIZE))
            .add_system(record_frame_time)
            .add_plugin(EguiPlugin)
            .add_system(draw_debug_fps_ui);
    }
}

fn record_frame_time(mut frame_buffer: ResMut<FrameTimeBuffer>, time: ResMut<Time>) {
    frame_buffer.record_frame(time.delta_seconds());
}

fn draw_debug_fps_ui(mut egui_context: ResMut<EguiContext>, frame_buffer: ResMut<FrameTimeBuffer>) {
    egui::Window::new("FPS").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Average MS: {:.2}", frame_buffer.average_ms()));
        ui.label(format!("FPS: {:.2}", frame_buffer.average_fps()));
    });
}
