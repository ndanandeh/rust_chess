use super::board::Board;
use super::types::color::Color;

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
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn new_game_test() {
        let game = Game::new();
        println!("{}", game.board);
    }
}