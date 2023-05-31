use crate::game::board_interaction::{BoardLocation, GameBoard, GamePiece, Player};

pub struct Game {
    pub board: GameBoard,
    pub current_player: Player,
    pub winning_sequence_length: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, winning_sequence_length: usize) -> Game {
        Game {
            board: GameBoard::new(width, height),
            current_player: Player::Player1,
            winning_sequence_length,
        }
    }

    pub fn place_piece(&mut self, column_index: usize) -> Result<BoardLocation, &'static str> {
        let location = self.board.place_piece(column_index, self.current_player)?;

        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };

        Ok(location)
    }

    pub fn check_if_winning_piece(
        &self,
        win_check_location: BoardLocation,
    ) -> Result<bool, &'static str> {
        const DIRECTIONS: [(usize, usize); 3] = [(1, 0), (0, 1), (1, 1)];

        if let GamePiece::PlayerPiece(player) = self.board[win_check_location] {
            for direction in DIRECTIONS.iter() {
                if self.check_if_winning_piece_in_direction(win_check_location, *direction, player) {
                    return Ok(true);
                }
            }
            return Ok(false);
        } else {
            return Err("No piece at location");
        }
    }

    fn check_if_winning_piece_in_direction(
        &self,
        win_check_location: BoardLocation,
        direction_vector: (usize, usize),
        player: Player,
    ) -> bool {
        let mut consecutive_pieces = 0;

        for direction_vector_scaling_factor in
            -(self.winning_sequence_length as isize)..=(self.winning_sequence_length as isize)
        {
            let row_index_isize = win_check_location.row_index as isize
                + direction_vector.0 as isize * direction_vector_scaling_factor;
            let column_index_isize = win_check_location.column_index as isize
                + direction_vector.1 as isize * direction_vector_scaling_factor;

            // Skip if indices are out of bounds
            if row_index_isize < 0 || column_index_isize < 0 {
                continue;
            }

            let row_index = row_index_isize as usize;
            let column_index = column_index_isize as usize;

            if let Some(GamePiece::PlayerPiece(piece_player)) =
                self.board.get_piece(column_index, row_index)
            {
                if *piece_player == player {
                    consecutive_pieces += 1;
                    if consecutive_pieces >= self.winning_sequence_length {
                        return true;
                    }
                } else {
                    consecutive_pieces = 0; // reset counter if we hit a piece that's not ours
                }
            } else {
                consecutive_pieces = 0;
            }
        }
        false
    }
}
