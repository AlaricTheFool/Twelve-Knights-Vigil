use super::*;

const POWER_DEPLETION_TICK_MS: u64 = 500;

pub struct PowerBarPlugin;

impl Plugin for PowerBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(normal_knight_power_scaling.run_in_state(GameMode::TDMode))
            .add_system(normal_knight_minigame_power_boost.run_in_state(GameMode::TDMode))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                normal_knight_ai_turn.run_in_state(GameMode::TDMode),
            );

        let mut fixed_stage = SystemStage::parallel();
        fixed_stage.add_system(deplete_power_bars.run_in_state(GameMode::TDMode));

        app.add_stage_before(
            CoreStage::Update,
            "power_bar_fixed",
            FixedTimestepStage::new(std::time::Duration::from_millis(POWER_DEPLETION_TICK_MS))
                .with_stage(fixed_stage),
        );
    }
}

#[derive(Component)]
pub struct PowerBar(u32);

impl PowerBar {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get_pct(&self) -> f32 {
        self.0 as f32 / 100.0
    }

    pub fn get_actual(&self) -> u32 {
        self.0
    }

    pub fn increase(&mut self, amount: u32) {
        self.0 = (self.0 + amount).clamp(0, 100);
    }

    pub fn decrease(&mut self, amount: u32) {
        self.0 = self.0.saturating_sub(amount).clamp(0, 100);
    }
}

fn normal_knight_ai_turn(mut game_query: Query<&mut tic_tac_toe::TicTacToe>) {
    game_query
        .iter_mut()
        .filter(|game| game.is_o_turn())
        .for_each(|mut game| {
            game.play_random_square();
        });
}

const MINIMUM_DAMAGE: i32 = 1;
const MAXIMUM_DAMAGE: i32 = 10;

const MAXIMUM_COOLDOWN: f32 = 1.0;
const MINIMUM_COOLDOWN: f32 = 0.1;

fn normal_knight_power_scaling(
    tower_query: Query<(Entity, &Knight, &PowerBar, &Cooldown), Changed<PowerBar>>,
    mut commands: Commands,
) {
    tower_query
        .iter()
        .filter(|(_, k, _, _)| **k == Knight::Normal)
        .for_each(|(e, _, power, cd)| {
            let new_damage = MINIMUM_DAMAGE
                + ((MAXIMUM_DAMAGE - MINIMUM_DAMAGE) as f32 * power.get_pct()).floor() as i32;
            let new_cooldown =
                MAXIMUM_COOLDOWN - (MAXIMUM_COOLDOWN - MINIMUM_COOLDOWN) * power.get_pct();

            commands
                .entity(e)
                .insert(cd.with_new_length(new_cooldown))
                .insert(Damage(new_damage as u32));
        });
}

fn normal_knight_minigame_power_boost(
    mut tower_query: Query<(Entity, &mut PowerBar, &tic_tac_toe::TicTacToe)>,
    mut commands: Commands,
) {
    use tic_tac_toe::*;
    tower_query.iter_mut().for_each(|(entity, mut bar, game)| {
        if let Some(game_result) = game.is_game_over() {
            match game_result {
                GameResult::Victory(player) => {
                    match player {
                        Player::X => bar.increase(25),
                        _ => {}
                    };
                }

                _ => {}
            }

            let mut new_game = TicTacToe::new();
            let mut gen_successful = false;

            while !gen_successful {
                info!("Generating tic tac toe game.");

                let iters = match bar.get_actual() {
                    81..=100 => 0,
                    40..=80 => 1,
                    _ => 3,
                };
                (0..iters).for_each(|_| {
                    new_game.play_random_square();
                    new_game.play_random_square();
                });

                if new_game.is_game_over().is_some() {
                    new_game = TicTacToe::new();
                } else {
                    gen_successful = true;
                }
            }
            commands.entity(entity).insert(new_game);
        }
    });
}

fn deplete_power_bars(mut bar_query: Query<&mut PowerBar>) {
    bar_query.iter_mut().for_each(|mut bar| {
        bar.decrease(1);
    });
}
