use super::types::piece::*;
use super::types::coordinate::*;
use super::types::color::*;

pub struct Board{
    // index as [rank = y coord = 1..8][file = x coord == a..h]
    board: [[Option<Piece>; 8]; 8]
}

impl Board {
    // Initialize a new board with pieces in the correct default spots
    pub fn new() -> Self {
        let black_pieces = [
            Some(Piece::new(PieceType::Rook, Color::Black)), Some(Piece::new(PieceType::Knight, Color::Black)), Some(Piece::new(PieceType::Bishop, Color::Black)),
            Some(Piece::new(PieceType::Queen, Color::Black)), Some(Piece::new(PieceType::King, Color::Black)), 
            Some(Piece::new(PieceType::Bishop, Color::Black)), Some(Piece::new(PieceType::Knight, Color::Black)), Some(Piece::new(PieceType::Rook, Color::Black))
        ];

        let white_pieces = [
            Some(Piece::new(PieceType::Rook, Color::White)), Some(Piece::new(PieceType::Knight, Color::White)), Some(Piece::new(PieceType::Bishop, Color::White)),
            Some(Piece::new(PieceType::Queen, Color::White)), Some(Piece::new(PieceType::King, Color::White)), 
            Some(Piece::new(PieceType::Bishop, Color::White)), Some(Piece::new(PieceType::Knight, Color::White)), Some(Piece::new(PieceType::Rook, Color::White))
        ];

        let black_pawns = [Some(Piece::new(PieceType::Pawn, Color::Black)); 8];
        let white_pawns = [Some(Piece::new(PieceType::Pawn, Color::White)); 8];

        let pieces_arr = [white_pieces, white_pawns, [None; 8], [None; 8], [None; 8], [None; 8], black_pieces, black_pawns];

    
        Board {board: pieces_arr}
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.board[square.rank as usize][square.file as usize]
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.board[square.rank as usize][square.file as usize] = Some(piece);
    }

    pub fn clear_square(&mut self, square: Square) {
        self.board[square.rank as usize][square.file as usize] = None;
    }

    pub fn is_empty(&self, square: Square) -> bool {
        self.board[square.rank as usize][square.file as usize].is_none()
    }

    pub fn is_enemy(&self, square: Square, color: Color) -> bool {
        match self.get_piece(square) {
            Some(piece) => {
                piece.get_color() != color
            },
            None => false
        }
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_str: String = String::new();
        for r in self.board.iter().rev() {
            for op in r {
                match op {
                    Some(piece) => { board_str += &format!(" {} ", piece);},
                    None =>  {board_str += &format!(" __ "); },
                }
            }
            board_str.push_str("\n")
        }
        write!(f, "{}", board_str)
    }
}