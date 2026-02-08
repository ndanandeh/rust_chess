use super::types::*;

pub struct Board{
    board: Vec<Vec<Square>>
}

impl Board {
    // Initialize a new board with pieces in the correct default spots
    pub fn new() -> Self {

        // Initialize the board vector (all empty - no pieces)
        let mut board_vec: Vec<Vec<Square>> = Vec::new();
        for i in 0..8 {
            board_vec.push(Vec::new());
            for j in 0..8 {
                board_vec[i].push(Square::new(i,j));
            }
        }
        let mut board = Board {board: board_vec};

        // place pieces
        // Place white pieces
        board.place_piece(Piece::new(PieceType::Rook, Color::White), Rank::ONE, File::A);
        board.place_piece(Piece::new(PieceType::Knight, Color::White), Rank::ONE, File::B);
        board.place_piece(Piece::new(PieceType::Bishop, Color::White), Rank::ONE, File::C);
        board.place_piece(Piece::new(PieceType::Queen, Color::White), Rank::ONE, File::D);
        board.place_piece(Piece::new(PieceType::King, Color::White), Rank::ONE, File::E);
        board.place_piece(Piece::new(PieceType::Bishop, Color::White), Rank::ONE, File::F);
        board.place_piece(Piece::new(PieceType::Knight, Color::White), Rank::ONE, File::G);
        board.place_piece(Piece::new(PieceType::Rook, Color::White), Rank::ONE, File::H);

        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::A);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::B);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::C);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::D);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::E);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::F);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::G);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), Rank::TWO, File::H);


        // Place black pieces
        board.place_piece(Piece::new(PieceType::Rook, Color::Black), Rank::EIGHT, File::A);
        board.place_piece(Piece::new(PieceType::Knight, Color::Black), Rank::EIGHT, File::B);
        board.place_piece(Piece::new(PieceType::Bishop, Color::Black), Rank::EIGHT, File::C);
        board.place_piece(Piece::new(PieceType::Queen, Color::Black), Rank::EIGHT, File::D);
        board.place_piece(Piece::new(PieceType::King, Color::Black), Rank::EIGHT, File::E);
        board.place_piece(Piece::new(PieceType::Bishop, Color::Black), Rank::EIGHT, File::F);
        board.place_piece(Piece::new(PieceType::Knight, Color::Black), Rank::EIGHT, File::G);
        board.place_piece(Piece::new(PieceType::Rook, Color::Black), Rank::EIGHT, File::H);

        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::A);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::B);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::C);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::D);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::E);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::F);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::G);
        board.place_piece(Piece::new(PieceType::Pawn, Color::Black), Rank::SEVEN, File::H);

        board
    } 

    fn get_square_ref_mut(&mut self, rank: Rank, file: File) -> &mut Square {
        &mut self.board[rank as usize][file as usize]
    }

    fn place_piece(&mut self, piece: Piece, rank: Rank, file: File) {
        let mut square = self.get_square_ref_mut(rank, file);
        square.place_piece(piece);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_str: String = String::new();
        for r in &self.board {
            for c in r {
                board_str += &format!(" {} ", c);
            }
            board_str.push_str("\n")
        }
        write!(f, "{}", board_str)
    }
}