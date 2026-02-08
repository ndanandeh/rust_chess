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

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl PieceType {
    pub fn char_rep(&self) -> char {
        match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Bishop => 'B',
            Self::Knight => 'N',
            Self::Pawn => 'P'
        }
    }
}

pub enum MoveError{
    IllegalMove,
    NoPiece
}

// Rows on the chess board (1-8)
#[derive(Default)]
pub enum Rank {
    #[default]
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT
}

impl TryFrom<usize> for Rank {
    type Error = &'static str; // Define a simple error type
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::ONE),
            1 => Ok(Rank::TWO),
            2 => Ok(Rank::THREE),
            3 => Ok(Rank::FOUR),
            4 => Ok(Rank::FIVE),
            5 => Ok(Rank::SIX),
            6 => Ok(Rank::SEVEN),
            7 => Ok(Rank::EIGHT),
            _ => Err("Invalid enum value"),
        }
    }
}

// Cols on the chess board (a-h)
#[derive(Default)]
pub enum File {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl TryFrom<usize> for File {
    type Error = &'static str; // Define a simple error type
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err("Invalid enum value"),
        }
    }
}

#[derive(Default)]
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
    pub fn pick_piece(mut self) -> Result<Piece, MoveError> {
        let result = self.piece.ok_or(MoveError::NoPiece);
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

pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    pub fn get_type(&self) -> &PieceType {
        &self.piece_type
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.color.char_rep(), self.piece_type.char_rep())
    }
}

pub struct Move {
    from: Square,
    to: Square
}