use super::coordinate::*;

#[derive(Debug)]
pub enum MoveError{
    NoPiece,
    WrongColor,
    IllegalMove,
    InvalidInput
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MoveCommand {
    pub from: Square,
    pub to: Square
}

impl MoveCommand {
    pub fn new(from: Square, to:Square) -> Self {
        MoveCommand {from, to}
    }

    pub fn new_alg(from: &str, to: &str) -> Result<MoveCommand, MoveError> {
        let from_square = Square::new(from).ok_or(MoveError::InvalidInput)?;
        let to_square = Square::new(to).ok_or(MoveError::InvalidInput)?;
        
        Ok(MoveCommand::new(from_square, to_square))
    }

}