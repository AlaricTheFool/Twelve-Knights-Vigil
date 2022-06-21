/// The highest level state of the program.
#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameState {
    MainMenu,
    TDMode,
}
