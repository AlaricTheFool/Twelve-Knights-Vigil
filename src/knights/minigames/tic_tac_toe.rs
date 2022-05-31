use super::*;

#[derive(Copy, Clone, PartialEq)]
enum BoardCell {
    Empty,
    Occupied(Player),
}

impl BoardCell {
    fn to_btn_text(&self) -> String {
        match *self {
            BoardCell::Empty => " ",
            BoardCell::Occupied(player) => match player {
                Player::X => "X",
                Player::O => "O",
            },
        }
        .to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameState {
    InProgress(Player),
    GameEnded(GameResult),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameResult {
    Victory(Player),
    Draw,
}

#[derive(Component, Clone, Copy)]
pub struct TicTacToe {
    game_state: GameState,
    board: [BoardCell; 9],
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            game_state: GameState::InProgress(Player::X),
            board: [BoardCell::Empty; 9],
        }
    }

    fn next_turn(&mut self) -> Result<(), TicTacToeError> {
        match self.game_state {
            GameState::InProgress(current_player) => {
                let new_player = match current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };

                self.game_state = GameState::InProgress(new_player);
                Ok(())
            }

            GameState::GameEnded(_) => Err(TicTacToeError::GameOver),
        }
    }

    pub fn play_square(&mut self, coord: Coordinate) -> Result<(), TicTacToeError> {
        match self.game_state {
            GameState::InProgress(current_player) => {
                let idx = TicTacToe::coord_to_idx(coord)?;

                if self.board[idx] == BoardCell::Empty {
                    self.board[idx] = BoardCell::Occupied(current_player);

                    if let Some(game_result) = self.is_game_over() {
                        self.game_state = GameState::GameEnded(game_result);
                    } else {
                        self.next_turn()?;
                    }

                    Ok(())
                } else {
                    Err(TicTacToeError::OccupiedSquare)
                }
            }

            GameState::GameEnded(_) => Err(TicTacToeError::GameOver),
        }
    }

    pub fn is_game_over(&self) -> Option<GameResult> {
        //Check Rows
        if let Some(winner) = (0..3).find_map(|y| {
            let start_idx = y * 3;
            let slice: &[BoardCell] = &self.board[start_idx..start_idx + 3];

            TicTacToe::check_winning_trio([slice[0], slice[1], slice[2]])
        }) {
            return Some(GameResult::Victory(winner));
        }

        // Check Columns
        if let Some(winner) = (0..3).find_map(|x| {
            let top = self.board[TicTacToe::coord_to_idx(Coordinate::new(x, 0)).unwrap()];
            let mid = self.board[TicTacToe::coord_to_idx(Coordinate::new(x, 1)).unwrap()];
            let bot = self.board[TicTacToe::coord_to_idx(Coordinate::new(x, 2)).unwrap()];

            TicTacToe::check_winning_trio([top, mid, bot])
        }) {
            return Some(GameResult::Victory(winner));
        }

        // Check Main Diagonal
        let tl = self.get_cell(0, 0).unwrap();
        let center = self.get_cell(1, 1).unwrap();
        let br = self.get_cell(2, 2).unwrap();

        if let Some(winner) = TicTacToe::check_winning_trio([tl, center, br]) {
            return Some(GameResult::Victory(winner));
        }

        // Check Reverse Diagonal
        let tr = self.get_cell(2, 0).unwrap();
        let bl = self.get_cell(0, 2).unwrap();

        if let Some(winner) = TicTacToe::check_winning_trio([tr, center, bl]) {
            return Some(GameResult::Victory(winner));
        }

        if !self.board.contains(&BoardCell::Empty) {
            Some(GameResult::Draw);
        }
        None
    }

    fn get_cell(&self, x: i32, y: i32) -> Result<BoardCell, TicTacToeError> {
        let idx = TicTacToe::coord_to_idx(Coordinate::new(x, y))?;
        Ok(self.board[idx])
    }

    fn coord_to_idx(coord: Coordinate) -> Result<usize, TicTacToeError> {
        if coord.x >= 0 && coord.x < 3 && coord.y >= 0 && coord.y < 3 {
            Ok((coord.x + (coord.y * 3)) as usize)
        } else {
            Err(TicTacToeError::OutOfBounds)
        }
    }

    fn check_winning_trio(cells: [BoardCell; 3]) -> Option<Player> {
        let all_eq = (cells[0] == cells[1]) && cells[0] == cells[2];

        if !all_eq {
            return None;
        }

        if let BoardCell::Occupied(winner) = cells[0] {
            Some(winner)
        } else {
            None
        }
    }

    pub fn get_as_string(&self) -> String {
        let mut result = "".to_string();
        self.board.iter().enumerate().for_each(|(idx, cell)| {
            if idx != 0 && idx % 3 == 0 {
                result += "\n";
            }

            let symbol = match *cell {
                BoardCell::Occupied(player) => match player {
                    Player::X => "X",
                    Player::O => "O",
                },
                _ => ".",
            };

            result += symbol;
        });

        result
    }

    pub fn render_egui(&self, ui: &mut Ui) -> Option<Self> {
        (0..3).find_map(|y| {
            let mut result = None;
            ui.horizontal(|ui| {
                result = (0..3).find_map(|x| {
                    let tile = self.get_cell(x, y).unwrap();
                    let btn_text = tile.to_btn_text();

                    if ui.button(btn_text).clicked() {
                        info!("Button Pushed! {x}, {y}");

                        let coord = Coordinate::new(x, y);
                        if let Ok(move_played_board) = self.with_next_move(coord) {
                            return Some(move_played_board);
                        }
                    }

                    None
                });
            });

            result
        })
    }

    fn with_next_move(&self, coord: Coordinate) -> Result<Self, TicTacToeError> {
        let mut new_board = self.clone();
        new_board.play_square(coord)?;
        Ok(new_board)
    }
}

#[derive(Debug, PartialEq)]
pub enum TicTacToeError {
    OccupiedSquare,
    OutOfBounds,
    GameOver,
}

impl std::fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_msg = match *self {
            _ => "This is an error.",
        };

        write!(f, "{error_msg}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let game = TicTacToe::new();
        let expected = "...\n...\n...";
        assert_eq!(game.get_as_string(), expected);
    }

    #[test]
    fn test_some_moves() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();
        game.play_square(Coordinate::new(1, 1))?;
        game.play_square(Coordinate::new(0, 0))?;
        game.play_square(Coordinate::new(2, 1))?;

        let expected = "O..\n.XX\n...";
        assert_eq!(game.get_as_string(), expected);
        Ok(())
    }

    #[test]
    fn test_invalid_moves() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        let out_of_bounds = game.play_square(Coordinate::new(3, -1));
        assert_eq!(out_of_bounds, Err(TicTacToeError::OutOfBounds));

        game.play_square(Coordinate::new(1, 1))?;
        let occupied = game.play_square(Coordinate::new(1, 1));

        assert_eq!(occupied, Err(TicTacToeError::OccupiedSquare));

        Ok(())
    }

    #[test]
    fn test_game_finished() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        (0..2).try_for_each(|x| {
            game.play_square(Coordinate::new(x, 0))?;
            game.play_square(Coordinate::new(x, 1))?;
            Ok(())
        })?;

        game.play_square(Coordinate::new(2, 0))?;

        assert_eq!(
            game.game_state,
            GameState::GameEnded(GameResult::Victory(Player::X))
        );

        let game_over_error = game.play_square(Coordinate::new(2, 2));
        assert_eq!(game_over_error, Err(TicTacToeError::GameOver));

        Ok(())
    }

    #[test]
    fn test_column_victory() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        (0..2).try_for_each(|y| {
            game.play_square(Coordinate::new(0, y))?;
            game.play_square(Coordinate::new(1, y))?;
            Ok(())
        })?;

        game.play_square(Coordinate::new(0, 2))?;

        assert_eq!(
            game.game_state,
            GameState::GameEnded(GameResult::Victory(Player::X))
        );

        Ok(())
    }

    #[test]
    fn test_o_victory() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        (0..2).try_for_each(|y| {
            game.play_square(Coordinate::new(0, y))?;
            game.play_square(Coordinate::new(1, y))?;
            Ok(())
        })?;

        game.play_square(Coordinate::new(2, 2))?;
        game.play_square(Coordinate::new(1, 2))?;

        assert_eq!(
            game.game_state,
            GameState::GameEnded(GameResult::Victory(Player::O))
        );

        Ok(())
    }

    #[test]
    fn test_diagonal_victory() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        game.play_square(Coordinate::new(0, 0))?;
        game.play_square(Coordinate::new(1, 0))?;
        game.play_square(Coordinate::new(1, 1))?;
        game.play_square(Coordinate::new(2, 0))?;
        game.play_square(Coordinate::new(2, 2))?;

        assert_eq!(
            game.game_state,
            GameState::GameEnded(GameResult::Victory(Player::X))
        );

        Ok(())
    }

    #[test]
    fn test_reverse_diagonal_victory() -> Result<(), TicTacToeError> {
        let mut game = TicTacToe::new();

        game.play_square(Coordinate::new(2, 0))?;
        game.play_square(Coordinate::new(1, 0))?;
        game.play_square(Coordinate::new(1, 1))?;
        game.play_square(Coordinate::new(0, 0))?;
        game.play_square(Coordinate::new(0, 2))?;

        assert_eq!(
            game.game_state,
            GameState::GameEnded(GameResult::Victory(Player::X))
        );

        Ok(())
    }
}
