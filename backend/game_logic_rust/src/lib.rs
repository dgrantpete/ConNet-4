#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameCellState {
    Player1 = 0,
    Player2 = 1,
    Empty = 2,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum Player {
    Player1 = 0,
    Player2 = 1,
}

#[repr(C)]
pub struct PlacePieceResult {
    pub is_winning_move: bool,
    pub points: i32,
}

#[derive(PartialEq, Debug)]
struct BoardCoordinates {
    row: usize,
    column: usize,
}

impl From<Player> for GameCellState {
    fn from(player: Player) -> Self {
        match player {
            Player::Player1 => GameCellState::Player1,
            Player::Player2 => GameCellState::Player2,
        }
    }
}

pub struct GameBoard {
    pub rows: usize,
    pub columns: usize,
    game_array: Vec<GameCellState>,
}

impl GameBoard {
    pub fn from_pointer(ptr: *mut GameCellState, rows: usize, columns: usize) -> Self {
        let game_array = unsafe { Vec::from_raw_parts(ptr, rows * columns, rows * columns) };
        GameBoard {
            rows,
            columns,
            game_array,
        }
    }

    pub fn get_cell_state(&self, row: usize, column: usize) -> Result<GameCellState, String> {
        if row >= self.rows || column >= self.columns {
            return Err(String::from("Index out of bounds"));
        }

        Ok(self.game_array[row * self.columns + column])
    }

    fn iter_column<'a>(&'a self, column_index: usize) -> impl Iterator<Item = GameCellState> + 'a {
        (0..self.rows).map(move |row_index| self.game_array[row_index * self.columns + column_index])
    }

    fn drop_in_piece(&mut self, column_index: usize, player: Player) -> Result<BoardCoordinates, String> {
        if column_index >= self.columns {
            return Err(String::from("Index out of bounds"));
        }

        let row_index = self.iter_column(column_index)
            .position(|cell_state| cell_state == GameCellState::Empty);

        if let Some(row_index) = row_index {
            self.set_cell_state(row_index, column_index, player.into())?;
            return Ok(BoardCoordinates { row: row_index, column: column_index });
        }

        Err(String::from("Column is full"))
    }

    fn get_contiguous(&self, row_index: usize, column_index: usize) -> Option<i32> {
        const DIRECTIONS: &[(isize, isize)] = &[(0, 1), (1, 0), (1, 1), (-1, 1)];
    
        let cell_state = self.get_cell_state(row_index, column_index).ok()?;

        if let GameCellState::Empty = cell_state {
            return None;
        }
    
        DIRECTIONS.iter()
            .map(|&(delta_row, delta_col)| {
                self.get_contiguous_in_direction(row_index as isize, column_index as isize, delta_row, delta_col, cell_state)
                    + self.get_contiguous_in_direction(row_index as isize, column_index as isize, -delta_row, -delta_col, cell_state)
                    + 1
            })
            .max()
    }
    
    fn get_contiguous_in_direction(&self, row_index: isize, column_index: isize, delta_row: isize, delta_col: isize, cell_state: GameCellState) -> i32 {
        let mut points = 0;
        let mut current_row = row_index + delta_row;
        let mut current_col = column_index + delta_col;
    
        loop {
            match self.get_cell_state(current_row as usize, current_col as usize) {
                Ok(state) if state == cell_state => {
                    points += 1;
                    current_row += delta_row;
                    current_col += delta_col;
                }
                _ => break,
            }
        }
    
        points
    }

    fn set_cell_state(&mut self, row: usize, column: usize, state: GameCellState) -> Result<(), String> {
        if row >= self.rows || column >= self.columns {
            return Err(String::from("Index out of bounds"));
        }

        self.game_array[row * self.columns + column] = state;
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn drop_in_piece(board: *mut GameCellState, rows: usize, columns: usize, column_index: usize, player: Player) -> PlacePieceResult {
    let mut game_board = GameBoard::from_pointer(board, rows, columns);
    let position = game_board.drop_in_piece(column_index, player);

    match position {
        Ok(position) => {
            let points = game_board.get_contiguous(position.row, position.column).unwrap_or(0);
            PlacePieceResult {
                is_winning_move: points >= 4,
                points,
            }
        }
        Err(_) => PlacePieceResult {
            is_winning_move: false,
            points: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell_state() {
        let game_array = vec![GameCellState::Empty, GameCellState::Player1, GameCellState::Player2];
        let board = GameBoard {
            rows: 3,
            columns: 1,
            game_array,
        };

        assert_eq!(board.get_cell_state(0, 0).unwrap(), GameCellState::Empty);
        assert_eq!(board.get_cell_state(1, 0).unwrap(), GameCellState::Player1);
        assert_eq!(board.get_cell_state(2, 0).unwrap(), GameCellState::Player2);
        assert!(board.get_cell_state(3, 0).is_err());
    }

    #[test]
    fn test_drop_in_piece() {
        let game_array = vec![GameCellState::Empty, GameCellState::Empty, GameCellState::Empty];
        let mut board = GameBoard {
            rows: 3,
            columns: 1,
            game_array,
        };

        assert_eq!(board.drop_in_piece(0, Player::Player1).unwrap(), BoardCoordinates { row: 0, column: 0 });
        assert_eq!(board.drop_in_piece(0, Player::Player2).unwrap(), BoardCoordinates { row: 1, column: 0 });
        assert_eq!(board.drop_in_piece(0, Player::Player1).unwrap(), BoardCoordinates { row: 2, column: 0 });
        assert!(board.drop_in_piece(0, Player::Player2).is_err());
    }

    #[test]
    fn test_get_contiguous() {
        let game_array = vec![
            GameCellState::Player1, GameCellState::Empty, GameCellState::Empty,
            GameCellState::Player1, GameCellState::Empty, GameCellState::Empty,
            GameCellState::Player1, GameCellState::Empty, GameCellState::Empty
        ];

        let board = GameBoard {
            rows: 3,
            columns: 3,
            game_array,
        };

        assert_eq!(board.get_contiguous(0, 0), Some(3));
        assert_eq!(board.get_contiguous(0, 1), None);
    }
}