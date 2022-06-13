use super::*;
use egui::*;

pub mod smoothie_mix;
pub mod tic_tac_toe;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

pub trait KnightMinigame<T = Self> {
    fn render_egui(&self, ui: &mut Ui) -> Option<T>;
    fn get_game_result(&self) -> Option<GameResult>;
}
