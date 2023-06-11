use std::ops::Index;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Clone, Copy)]
pub enum GamePiece {
    Empty,
    PlayerPiece(Player),
}

#[derive(Clone, Copy)]
pub struct BoardLocation {
    pub row_index: usize,
    pub column_index: usize,
}

pub struct GameBoard {
    board: Vec<Vec<GamePiece>>,
}

impl GameBoard {
    pub fn new(width: usize, height: usize) -> GameBoard {
        GameBoard {
            board: vec![vec![GamePiece::Empty; height]; width],
        }
    }

    pub fn place_piece(
        &mut self,
        column_index: usize,
        player: Player,
    ) -> Result<BoardLocation, &'static str> {
        if column_index >= self.board.len() {
            return Err("Invalid column index");
        }

        for (row_index, piece) in self.board[column_index].iter_mut().enumerate() {
            if matches!(piece, GamePiece::Empty) {
                *piece = GamePiece::PlayerPiece(player);
                return Ok(BoardLocation {
                    row_index,
                    column_index,
                });
            }
        }

        Err("Column is full")
    }

    pub fn get_width(&self) -> usize {
        self.board.len()
    }

    pub fn get_height(&self) -> usize {
        self.board.first().unwrap().len()
    }

    pub fn get_piece(&self, column_index: usize, row_index: usize) -> Option<&GamePiece> {
        self.board.get(column_index).and_then(|column| column.get(row_index))
    }
}

impl Index<BoardLocation> for GameBoard {
    type Output = GamePiece;

    fn index(&self, index: BoardLocation) -> &Self::Output {
        &self.board[index.column_index][index.row_index]
    }
}
