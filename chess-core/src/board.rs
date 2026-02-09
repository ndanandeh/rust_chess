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

    // Executes the provided move command on the board
    // Does not perform any validation. Validation is expected ot be performed at the game level
    // If the from square is empty, then something has gone wrong.
    pub fn move_piece(&mut self, validated_move: MoveCommand) {
        let piece = self.get_piece(validated_move.from).unwrap();        
        self.set_piece(validated_move.to, piece);
        self.clear_square(validated_move.from);
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.board[square.rank as usize][square.file as usize]
    }

    fn set_piece(&mut self, square: Square, piece: Piece) {
        self.board[square.rank as usize][square.file as usize] = Some(piece);
    }

    fn clear_square(&mut self, square: Square) {
        self.board[square.rank as usize][square.file as usize] = None;
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