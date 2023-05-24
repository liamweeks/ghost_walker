use crate::prelude::*;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![None; 8]; 8];

        // Place white pieces
        board[0][0] = Some(Piece::Rook);
        board[0][1] = Some(Piece::Knight);
        board[0][2] = Some(Piece::Bishop);
        board[0][3] = Some(Piece::Queen);
        board[0][4] = Some(Piece::King);
        board[0][5] = Some(Piece::Bishop);
        board[0][6] = Some(Piece::Knight);
        board[0][7] = Some(Piece::Rook);

        for i in 0..8 {
            board[1][i] = Some(Piece::Pawn);
        }

        // Place black pieces
        board[7][0] = Some(Piece::Rook);
        board[7][1] = Some(Piece::Knight);
        board[7][2] = Some(Piece::Bishop);
        board[7][3] = Some(Piece::Queen);
        board[7][4] = Some(Piece::King);
        board[7][5] = Some(Piece::Bishop);
        board[7][6] = Some(Piece::Knight);
        board[7][7] = Some(Piece::Rook);

        for i in 0..8 {
            board[6][i] = Some(Piece::Pawn);
        }

        return Self { board };
    }
}
