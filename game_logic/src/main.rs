mod game;

use game::board_interaction::{BoardLocation, GameBoard, GamePiece, Player};
use game::game_logic::Game;
use itertools::Itertools;

fn main() {
    let mut game = Game::new(7, 6, 4);

    loop {
        println!("{}", generate_board_representation(&game.board));

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let column_index = input.trim().parse::<usize>().unwrap();

        match game.place_piece(column_index) {
            Ok(location) => {
                if game.check_if_winning_piece(location).unwrap() {
                    println!("Player {:?} won!", game.current_player);
                    break;
                }
            }
            Err(error_message) => {
                println!("{}", error_message);
            }
        }
    }
}

fn generate_board_representation(board: &GameBoard) -> String {
    (0..board.get_height())
        .rev()
        .map(|row_index| {
            (0..board.get_width())
                .map(|column_index| {
                    match board[BoardLocation {
                        row_index,
                        column_index,
                    }] {
                        GamePiece::Empty => '𐤏',
                        GamePiece::PlayerPiece(Player::Player1) => '◉',
                        GamePiece::PlayerPiece(Player::Player2) => '◎',
                    }
                })
                .join(" ")
        })
        .join("\n")
}
