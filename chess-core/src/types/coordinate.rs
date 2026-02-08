// Rows on the chess board (1-8)
#[derive(Copy, Clone)]
pub enum Rank {
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

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self as usize) + 1)
    }
}

// Cols on the chess board (a-h)
pub enum File {
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

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Square {
    rank: Rank,
    file: File,
}


impl Square {
    pub fn new(rank_i: usize, file_i: usize) -> Self {
        Square {rank: Rank::try_from(rank_i).unwrap(), file: File::try_from(file_i).unwrap() }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.file)
    }
}