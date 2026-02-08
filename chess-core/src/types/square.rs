use super::piece::*;
use super::coordinate::*;

pub struct Square {
    piece: Option<Piece>,
    rank: Rank,
    file: File,
}

impl Square {
    pub fn new(rank_i: usize, file_i: usize) -> Self {
        Square { piece: None, rank: Rank::try_from(rank_i).unwrap(), file: File::try_from(file_i).unwrap() }
    }

    pub fn peek_piece(&self) -> &Option<Piece> {
        &self.piece
    }

    // Gives up ownership of the piece on this square
    pub fn pick_piece(mut self) -> Option<Piece> {
        let result = self.piece;
        self.piece = None;
        result
    }

    // Give ownership of a piece to this square
    // TODO: Consider validation to see if the square is already filled
    pub fn place_piece(&mut self, piece: Piece) {
        self.piece = Some(piece); 
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let square_str = match &self.piece {
            Some(piece) => format!("{}", piece),
            None => "__".to_string()
        };
        write!(f, "{}", square_str)
    }
}