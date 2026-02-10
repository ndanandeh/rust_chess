use crate::types::move_command::MoveError;

use super::board::Board;
use super::types::color::Color;
use super::types::move_command::MoveCommand;
use super::types::piece::Piece;

use super::*;

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

    // Fake algebraic notation for now... 
    pub fn play_move(&mut self, from: &str, to: &str) -> Result<(), MoveError> {
        self.play_move_internal(MoveCommand::new_alg(from, to)?)
    }

    pub fn play_move_internal(&mut self, move_command: MoveCommand) -> Result<(), MoveError> {
        // get piece and return error if no piece exists
        let piece = self.board.get_piece(move_command.from).ok_or(MoveError::NoPiece)?;
        // return error if the piece we've selected is the wrong color
        if piece.get_color() != self.turn {
            return Err(MoveError::WrongColor);
        }
        // generate legal moves
        let legal_moves = movegen::generate_valid_moves(&self.board, self.turn);
        // if move not in legal moves error
        if !legal_moves.contains(&move_command) {
            return Err(MoveError::IllegalMove);
        }
        // apply move
        self.apply_validated_move(move_command, piece);

        // TODO: Consider Updating Game Status + terminating game if necessary

        // switch side
        self.switch_sides();
        Ok(())
    }

    fn apply_validated_move(&mut self, validated_move: MoveCommand, piece: Piece) {
        // TODO: keep track of enpassant and castle eligibility 
        self.board.clear_square(validated_move.from);
        // TODO: consider piece promotion here before setting
        self.board.set_piece(validated_move.to, piece);
    }

    fn switch_sides(&mut self) {
        match self.turn  {
            Color::White => {self.turn = Color::Black;},
            Color::Black => {self.turn = Color::White;}
        }
    }

    // pub fn get_valid_moves(&self, from_square: Square) -> Result<Vec<Square>, MoveError> {

    // }
}

#[cfg(test)]
mod test{

    use super::*;
    use crate::types::coordinate::Square;

    // #[test]
    // fn new_game_test() {
    //     let game = Game::new();
    //     println!("{}", game.board);
    // }

    #[test]
    fn move_test() {
        play_scholars_mate();
    }

    fn play_scholars_mate() {
        let mut game = Game::new();
        move_and_print(&mut game, "e2", "e4");
        move_and_print(&mut game, "e7", "e5");

        move_and_print(&mut game, "d1", "h5");
        move_and_print(&mut game, "b8", "c6");

        move_and_print(&mut game, "f1", "c4");
        move_and_print(&mut game, "g8", "f6");

        move_and_print(&mut game, "h5", "f7");
    }

    fn move_and_print(game: &mut Game, from: &str, to: &str) {
        game.play_move(from, to).unwrap();
        println!("{}", game.board);
        println!("------------------------------------------------------");
    }
}