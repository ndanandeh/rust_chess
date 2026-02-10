use std::collections::HashSet;

use crate::board::Board;
use crate::types::color::Color;
use crate::types::coordinate::Square;
use crate::types::piece::Piece;
use crate::types::piece::PieceType;
use crate::types::move_command::MoveCommand;



pub fn generate_valid_moves(board: &Board, color: Color) -> HashSet<MoveCommand> {
    let mut valid_moves = HashSet::new();

    for square in Square::get_all_squares() {
        let piece = board.get_piece(square);
        if piece.is_none() || piece.unwrap().get_color() != color {
            continue;
        }
        let piece = piece.unwrap();
        let candidate_moves = get_candidate_moves(board, square, piece);
        for candidate in candidate_moves {
            //TODO: Verify move doesn't leave in check
            valid_moves.insert(candidate);
        }
    }

    valid_moves
}

fn get_candidate_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    match piece.get_type() {
        PieceType::Pawn => pawn_moves(board, square, piece),
        PieceType::Bishop => bishop_moves(board, square, piece),
        PieceType::Knight => knight_moves(board, square, piece),
        PieceType::Rook => rook_moves(board, square, piece),
        PieceType::Queen => queen_moves(board, square, piece),
        PieceType::King => king_moves(board, square, piece),
    }
}

fn pawn_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();
    
    let direction = if piece.get_color() == Color::Black { -1 } else { 1 };
    let start_rank = if piece.get_color() == Color::Black {6} else {1};

    if let Some(forwards) = square.neighbor(direction, 0) && board.is_empty(forwards) {
        moves.insert(MoveCommand::new(square, forwards));
    }

    if square.rank == start_rank {
        if let Some(two_forwards) = square.neighbor(direction*2, 0) && board.is_empty(two_forwards) {
            moves.insert(MoveCommand::new(square, two_forwards));
        }
    }

    let diagonal_offsets = [(direction, 1), (direction, -1)];
    for (dr, df) in diagonal_offsets {
        if let Some(diagonal) = square.neighbor(dr, df) && board.is_enemy(diagonal, piece.get_color()) {
            moves.insert(MoveCommand::new(square, diagonal));
        }
    }
    moves
}

fn bishop_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();

    let directions = [(1,1), (-1,-1), (1,-1), (-1,1)];
    for (dr, df) in directions {
        let (mut r, mut f) = (dr, df);
        while let Some(diagonal) = square.neighbor(r, f) {
            if !board.is_empty(diagonal) {
                if board.is_enemy(diagonal, piece.get_color()) {
                    moves.insert(MoveCommand::new(square, diagonal));
                }
                break;
            }
            moves.insert(MoveCommand::new(square, diagonal));
            r += dr;
            f += df;
        }
    }

    moves
}

fn knight_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();

    let directions = [(2,1), (2,-1), (1,2), (1,-2), (-2,1), (-2,-1), (-1,2), (-1,-2)];
    for (dr, df) in directions {
        if let Some(move_sq) = square.neighbor(dr, df) {
            if board.is_empty(move_sq) || board.is_enemy(move_sq, piece.get_color()) {
                moves.insert(MoveCommand::new(square, move_sq));
            }
        }
    }
    moves
}

fn rook_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();

    let directions = [(1,0), (-1,0), (0,-1), (0,1)];
    for (dr,df) in directions {
        let (mut r,mut f) = (dr, df);
        while let Some(straight) = square.neighbor(r, f) {
            if !board.is_empty(straight) {
                if board.is_enemy(straight, piece.get_color()) {
                    moves.insert(MoveCommand::new(square, straight));
                }
                break;
            }
            moves.insert(MoveCommand::new(square, straight));
            r += dr;
            f += df;
        }
    }

    moves
}

fn queen_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();

    moves.extend(bishop_moves(board, square, piece));
    moves.extend(rook_moves(board, square, piece));

    moves
}

fn king_moves(board: &Board, square: Square, piece: Piece) -> HashSet<MoveCommand> {
    let mut moves = HashSet::new();

    let directions = [(1,0), (1,1), (1,-1), (0,1), (0,-1), (-1,0), (-1,1), (-1,-1)];
    for (dr, df) in directions {
        if let Some(move_sq) = square.neighbor(dr, df) {
            if board.is_empty(move_sq) || board.is_enemy(move_sq, piece.get_color()) {
                moves.insert(MoveCommand::new(square, move_sq));
            }
        }
    }

    moves
}
