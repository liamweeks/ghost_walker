use crate::prelude::*;

#[derive(Debug)]
pub struct Board<'a> {
    pub board: Vec<Vec<Option<Piece<'a>>>>,
}

impl Board<'_> {
    pub fn new() -> Self {
        let mut board = vec![vec![None; 8]; 8];

        // Place white pieces
        board[0][0] = Some(Piece::Rook("black", 5));
        board[0][1] = Some(Piece::Knight("black", 3));
        board[0][2] = Some(Piece::Bishop("black", 3));
        board[0][3] = Some(Piece::Queen("black", 9));
        board[0][4] = Some(Piece::King("black", 100));
        board[0][5] = Some(Piece::Bishop("black", 3));
        board[0][6] = Some(Piece::Knight("black", 3));
        board[0][7] = Some(Piece::Rook("black", 5));

        for i in 0..8 {
            board[1][i] = Some(Piece::Pawn("black", 1));
        }

        // Place black pieces
        board[7][0] = Some(Piece::Rook("white", 5));
        board[7][1] = Some(Piece::Knight("white", 3));
        board[7][2] = Some(Piece::Bishop("white", 3));
        board[7][3] = Some(Piece::Queen("white", 9));
        board[7][4] = Some(Piece::King("white", 100));
        board[7][5] = Some(Piece::Bishop("white", 3));
        board[7][6] = Some(Piece::Knight("white", 3));
        board[7][7] = Some(Piece::Rook("white", 5));

        for i in 0..8 {
            board[6][i] = Some(Piece::Pawn("white", 1));
        }

        return Self { board };
    }
}
