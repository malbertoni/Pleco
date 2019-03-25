//! The minimax algorithm.
use super::*;
use board::*;

pub fn minimax(board: &mut Board, depth: u16) -> ScoringMove {
    if depth == 0 {
        return eval_board(board);
    }

    let moves = board.generate_scoring_moves();
    if moves.is_empty() {
        if board.in_check() {
            return ScoringMove::blank(-MATE_V);
        } else {
            return ScoringMove::blank(DRAW_V);
        }
    }

    moves
        .into_iter()
        .map(|mut m: ScoringMove| {
            board.apply_move(m.bit_move);
            m.score = -minimax(board, depth - 1).score;
            board.undo_move();
            m
        })
        .max()
        .unwrap()
}
