use super::square::*;

pub enum MoveError{
    IllegalMove,
}
pub struct MoveCommand {
    from: Square,
    to: Square
}