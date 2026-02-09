pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

// Rows on the chess board (1-8)
#[derive(Copy, Clone, PartialEq, Eq)]
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

impl Rank {
    pub fn north(self) -> Option<Self> {
        Rank::try_from((self as usize) + 1).ok()
    }

    pub fn south(self) -> Option<Self> {
        let i_val = self as usize;
        if i_val == 0 {
            return None;
        }
        Rank::try_from(i_val -1).ok()
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self as usize) + 1)
    }
}

// Cols on the chess board (a-h)
#[derive(Copy, Clone, PartialEq, Eq)]
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

impl File {
    pub fn east(self) -> Option<Self> {
        File::try_from((self as usize) + 1).ok()
    }

    pub fn west(self) -> Option<Self> {
        let i_val = self as usize;
        if i_val == 0 {
            return None;
        }
        File::try_from(i_val -1).ok()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}


impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Square {file, rank}
    }

    pub fn neighbor(self, direction: Direction) -> Option<Self> {
        match(direction) {
            Direction::N => {
                let rank = self.rank.north()?;
                Some(Square::new(self.file, rank))
            },
            Direction::NE => {
                let rank = self.rank.north()?;
                let file = self.file.east()?;
                Some(Square::new(file, rank))
            },
            Direction::E => {
                let file = self.file.east()?;
                Some(Square::new(file, self.rank))
            },
            Direction::SE => {
                let rank = self.rank.south()?;
                let file = self.file.east()?;
                Some(Square::new(file, rank))
            },
            Direction::S => {
                let rank = self.rank.south()?;
                Some(Square::new(self.file, rank))
            },
            Direction::SW => {
                let rank = self.rank.south()?;
                let file = self.file.west()?;
                Some(Square::new(file, rank))
            },
            Direction::W => {
                let file = self.file.west()?;
                Some(Square::new(file, self.rank))
            },
            Direction::NW => {
                let rank = self.rank.north()?;
                let file = self.file.west()?;
                Some(Square::new(file, rank))
            },
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}