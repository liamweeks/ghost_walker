use crate::prelude::*;

#[derive(Debug)]
pub struct Board<'a> {
    pub board: Vec<Vec<Option<Piece<'a>>>>,
}

impl Board<'_> {
    pub fn new() -> Self {
        let mut board = vec![vec![None; 8]; 8];

        // Place white pieces
        board[0][0] = Some(Piece::Black("rook", 5));
        board[0][1] = Some(Piece::Black("knight", 3));
        board[0][2] = Some(Piece::Black("bishop", 3));
        board[0][3] = Some(Piece::Black("queen", 9));
        board[0][4] = Some(Piece::Black("king", 100));
        board[0][5] = Some(Piece::Black("bishop", 3));
        board[0][6] = Some(Piece::Black("knight", 3));
        board[0][7] = Some(Piece::Black("rook", 5));

        for i in 0..8 {
            board[1][i] = Some(Piece::Black("pawn", 1));
        }

        // Place black pieces
        board[7][0] = Some(Piece::White("rook", 5));
        board[7][1] = Some(Piece::White("knight", 3));
        board[7][2] = Some(Piece::White("bishop", 3));
        board[7][3] = Some(Piece::White("queen", 9));
        board[7][4] = Some(Piece::White("king", 100));
        board[7][5] = Some(Piece::White("bishop", 3));
        board[7][6] = Some(Piece::White("knight", 3));
        board[7][7] = Some(Piece::White("rook", 5));

        for i in 0..8 {
            board[6][i] = Some(Piece::White("pawn", 1));
        }

        return Self { board };
    }
}
