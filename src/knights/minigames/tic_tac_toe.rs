use super::*;

#[derive(Copy, Clone, PartialEq)]
enum BoardCell {
    Empty,
    Occupied(Player),
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

enum GameState {
    InProgress(Player),
    GameEnded(GameResult),
}

enum GameResult {
    Victory(Player),
    Draw,
}

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
                    self.next_turn()?;
                    Ok(())
                } else {
                    Err(TicTacToeError::OccupiedSquare)
                }
            }

            GameState::GameEnded(_) => Err(TicTacToeError::GameOver),
        }
    }

    fn coord_to_idx(coord: Coordinate) -> Result<usize, TicTacToeError> {
        if coord.x >= 0 && coord.x < 3 && coord.y >= 0 && coord.y < 3 {
            Ok((coord.x + (coord.y * 3)) as usize)
        } else {
            Err(TicTacToeError::OutOfBounds)
        }
    }
}

pub enum TicTacToeError {
    OccupiedSquare,
    OutOfBounds,
    GameOver,
}
