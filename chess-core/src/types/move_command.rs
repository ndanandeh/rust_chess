use super::coordinate::*;

#[derive(Debug)]
pub enum MoveError{
    NoPiece,
    IllegalMove,
}

#[derive(Copy, Clone)]
pub struct MoveCommand {
    pub from: Square,
    pub to: Square
}

impl MoveCommand {
    pub fn new(from: Square, to:Square) -> Self {
        MoveCommand {from, to}
    }
}