use super::*;

pub struct PowerBarPlugin;

impl Plugin for PowerBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(normal_knight_power_scaling.run_in_state(GameMode::TDMode))
            .add_system(normal_knight_minigame_power_boost.run_in_state(GameMode::TDMode));
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
        self.0 = (self.0 + amount).clamp(0, 100);
    }
}

const MINIMUM_DAMAGE: i32 = 1;
const MAXIMUM_DAMAGE: i32 = 10;

const MAXIMUM_COOLDOWN: f32 = 1.0;
const MINIMUM_COOLDOWN: f32 = 0.1;

fn normal_knight_power_scaling(
    tower_query: Query<(Entity, &Knight, &PowerBar), Changed<PowerBar>>,
    mut commands: Commands,
) {
    tower_query
        .iter()
        .filter(|(_, k, _)| **k == Knight::Normal)
        .for_each(|(e, _, power)| {
            let new_damage = MINIMUM_DAMAGE
                + ((MAXIMUM_DAMAGE - MINIMUM_DAMAGE) as f32 * power.get_pct()).floor() as i32;
            let new_cooldown =
                MAXIMUM_COOLDOWN - (MAXIMUM_COOLDOWN - MINIMUM_COOLDOWN) * power.get_pct();

            commands
                .entity(e)
                .insert(Cooldown::new(new_cooldown))
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
                        Player::X => bar.increase(5),
                        _ => {}
                    };
                }

                _ => {}
            }

            commands.entity(entity).insert(TicTacToe::new());
        }
    });
}
