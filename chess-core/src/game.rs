use crate::types::coordinate::*;
use crate::types::move_command::MoveError;

use super::board::Board;
use super::types::color::Color;
use super::types::move_command::MoveCommand;

use super::movegen::*;

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            turn: Color::White
        }
    }

    pub fn make_move(&mut self, move_command: MoveCommand) -> Result<(), MoveError> {
        let valid_moves = generate_valid_moves(move_command.from, &self.board)?;

        if (valid_moves.contains(&move_command.to)) {
            self.board.move_piece(move_command);
            Ok(())
        } else {
            Err(MoveError::IllegalMove)
        }
    }

    // pub fn get_valid_moves(&self, from_square: Square) -> Result<Vec<Square>, MoveError> {

    // }
}

#[cfg(test)]
mod test{

    use super::*;

    // #[test]
    // fn new_game_test() {
    //     let game = Game::new();
    //     println!("{}", game.board);
    // }

    #[test]
    fn move_test() {
        let mut game = Game::new();
        let move_command = MoveCommand::new(Square::new(File::A, Rank::TWO), Square::new(File::A, Rank::THREE));
        game.make_move(move_command).unwrap();
        println!("{}", game.board);
    }
}