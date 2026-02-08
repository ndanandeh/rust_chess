#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn char_rep(&self) -> char {
        match self {
            Self::White => 'w',
            Self::Black => 'b',
        }
    }
}