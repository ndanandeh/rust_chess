const RANKS: std::ops::Range<i8> = 0..8;
const FILES: std::ops::Range<i8> = 0..8;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}


impl Square {

    pub fn get_all_squares() -> Vec<Self> {
        let mut vec  = Vec::new();

        for rank in RANKS {
            for file in FILES {
                if let Some(sq) = Square::new_raw(rank, file) {
                    vec.push(sq);  
                }
            }
        }
        vec
    }
    
    pub fn new_raw(rank: i8, file: i8) -> Option<Self> {
        if !RANKS.contains(&rank) {
            None
        } else if !FILES.contains(&file) {
            None
        } else {
            Some(Square {rank, file})
        }
    }

    // Construct square from algebraic notation (e.g. a1, c7)
    pub fn new(alg: &str) -> Option<Self> {
        if alg.len() != 2 {
            return None;
        }

        let file_char = Square::get_file_ind(alg.chars().nth(0)?)?;
        let rank_char = Square::get_rank_ind(alg.chars().nth(1)?)?;

        Square::new_raw(rank_char, file_char)
    }

    pub fn neighbor(self, dr: i8, df: i8) -> Option<Self> {
        Square::new_raw(self.rank + dr, self.file + df)
    }

    fn get_file_ind(c: char) -> Option<i8> {
        match c.to_ascii_lowercase() {
            'a' => Some(0),
            'b' => Some(1),
            'c' => Some(2),
            'd' => Some(3),
            'e' => Some(4),
            'f' => Some(5),
            'g' => Some(6),
            'h' => Some(7),
            _ => None
        }
    }

    fn get_rank_ind(c: char) -> Option<i8> {
        match c {
            '1' => Some(0),
            '2' => Some(1),
            '3' => Some(2),
            '4' => Some(3),
            '5' => Some(4),
            '6' => Some(5),
            '7' => Some(6),
            '8' => Some(7),
            _ => None
        }
    }
    
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}