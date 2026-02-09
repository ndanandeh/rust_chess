use crate::types::color::Color;
use crate::types::move_command::MoveError;

use super::board::*;
use super::types::coordinate::*;
use super::types::piece::*;


pub fn generate_valid_moves(square: Square, board: &Board) -> Result<Vec<Square>, MoveError> {
    let Some(piece) = board.get_piece(square) else {
        return Err(MoveError::NoPiece);
    };

    match piece.get_type()  {
        PieceType::Pawn => {
            Ok(generate_valid_pawn_moves(square, board,  piece.get_color()))
        }
        _ => {
             Ok(Vec::new())
        }
    }
}


fn generate_valid_pawn_moves(square: Square, board: &Board, color: Color) -> Vec<Square> {
    let mut vec = Vec::new();
    // forward one space if unoccupied
    let forwards = match color {
        Color::White => {
            square.neighbor(Direction::N)
        },
        Color::Black => {
            square.neighbor(Direction::S)
        }
    };
    if !forwards.is_none() && board.get_piece(forwards.unwrap()).is_none() {
        vec.push(forwards.unwrap());
    }
    // forward two spaces if both unoccupied and at start
    let forwards = match color {
        Color::White => {
            square.neighbor(Direction::N).and_then(|s| s.neighbor(Direction::N))
        },
        Color::Black => {
            square.neighbor(Direction::S).and_then(|s| s.neighbor(Direction::S))
        }
    };
    if !forwards.is_none() && board.get_piece(forwards.unwrap()).is_none() {
        vec.push(forwards.unwrap());
    }

    // forward diagonally if occupied
    let forwards1 = match color {
        Color::White => {
            square.neighbor(Direction::NE)
        },
        Color::Black => {
            square.neighbor(Direction::SE)
        }
    };
    if !forwards1.is_none() && !board.get_piece(forwards1.unwrap()).is_none() {
        vec.push(forwards1.unwrap());
    }

    let forwards2 = match color {
        Color::White => {
            square.neighbor(Direction::NW)
        },
        Color::Black => {
            square.neighbor(Direction::SW)
        }
    };
    if !forwards2.is_none() && !board.get_piece(forwards2.unwrap()).is_none() {
        vec.push(forwards2.unwrap());
    }
    vec
}