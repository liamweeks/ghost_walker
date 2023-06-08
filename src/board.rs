use crate::prelude::*;

#[derive(Debug)]
pub struct Board<'a> {
    pub board: Vec<Vec<Piece<'a>>>,
}

impl Board<'_> {
    pub fn new() -> Self {
        let mut board = vec![vec![Piece::Empty; 8]; 8];

        // Place white pieces
        board[0][0] = Piece::Black("rook", 5);
        board[0][1] = Piece::Black("knight", 3);
        board[0][2] = Piece::Black("bishop", 3);
        board[0][3] = Piece::Black("queen", 9);
        board[0][4] = Piece::Black("king", 100);
        board[0][5] = Piece::Black("bishop", 3);
        board[0][6] = Piece::Black("knight", 3);
        board[0][7] = Piece::Black("rook", 5);

        for i in 0..8 {
            board[1][i] = Piece::Black("pawn", 1);
        }

        // Place black pieces
        board[7][0] = Piece::White("rook", 5);
        board[7][1] = Piece::White("knight", 3);
        board[7][2] = Piece::White("bishop", 3);
        board[7][3] = Piece::White("queen", 9);
        board[7][4] = Piece::White("king", 100);
        board[7][5] = Piece::White("bishop", 3);
        board[7][6] = Piece::White("knight", 3);
        board[7][7] = Piece::White("rook", 5);

        for i in 0..8 {
            board[6][i] = Piece::White("pawn", 1);
        }

        return Self { board };
    }
}
