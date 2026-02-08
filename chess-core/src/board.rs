use super::types::piece::*;
use super::types::coordinate::*;
use super::types::color::*;
use super::types::move_command::*;

pub struct Board{
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

    fn execute_move(&mut self, move_command: MoveCommand) {

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