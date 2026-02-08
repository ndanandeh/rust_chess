use super::coordinate::*;

pub enum MoveError{
    IllegalMove,
}
pub struct MoveCommand {
    pub from: Square,
    pub to: Square
}