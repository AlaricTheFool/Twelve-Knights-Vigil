use super::*;

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
pub enum WaveState {
    Preparation,
    WaveInProgress,
}

pub struct WaveInfo {
    pub remaining_enemies_to_spawn: i32,
}

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(WaveState::Preparation)
            .add_enter_system(GameMode::TDMode, initialize_wavestate)
            .add_enter_system(WaveState::WaveInProgress, initialize_wave_data)
            .add_system(end_wave.run_in_state(WaveState::WaveInProgress))
            .add_system(
                render_between_wave_ui
                    .run_in_state(WaveState::Preparation)
                    .run_in_state(GameMode::TDMode),
            );
    }
}

fn initialize_wavestate(mut commands: Commands) {
    commands.insert_resource(NextState(WaveState::Preparation));
}

fn initialize_wave_data(mut commands: Commands) {
    let new_wave = WaveInfo {
        remaining_enemies_to_spawn: thread_rng().gen_range(5..15),
    };

    commands.insert_resource(new_wave);
}

fn render_between_wave_ui(mut egui_context: ResMut<EguiContext>, mut commands: Commands) {
    egui::Window::new("Preparation Phase").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Begin Wave").clicked() {
            commands.insert_resource(NextState(WaveState::WaveInProgress));
        }
    });
}

fn end_wave(enemy_query: Query<&Enemy>, wave: Res<WaveInfo>, mut commands: Commands) {
    if wave.remaining_enemies_to_spawn < 1 && enemy_query.is_empty() {
        commands.insert_resource(NextState(WaveState::Preparation));
    }
}
