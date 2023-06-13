use crate::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<Piece>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![Piece::Empty; 8]; 8];

        // Place white pieces
        board[0][0] = Piece::Black(Warrior::Rook, 5);
        board[0][1] = Piece::Black(Warrior::Knight, 3);
        board[0][2] = Piece::Black(Warrior::Bishop, 3);
        board[0][3] = Piece::Black(Warrior::Queen, 9);
        board[0][4] = Piece::Black(Warrior::King, 100);
        board[0][5] = Piece::Black(Warrior::Bishop, 3);
        board[0][6] = Piece::Black(Warrior::Knight, 3);
        board[0][7] = Piece::Black(Warrior::Rook, 5);

        for i in 0..8 {
            board[1][i] = Piece::Black(Warrior::Pawn, 1);
        }

        // Place black pieces
        board[7][0] = Piece::White(Warrior::Rook, 5);
        board[7][1] = Piece::White(Warrior::Knight, 3);
        board[7][2] = Piece::White(Warrior::Bishop, 3);
        board[7][3] = Piece::White(Warrior::Queen, 9);
        board[7][4] = Piece::White(Warrior::King, 100);
        board[7][5] = Piece::White(Warrior::Bishop, 3);
        board[7][6] = Piece::White(Warrior::Knight, 3);
        board[7][7] = Piece::White(Warrior::Rook, 5);

        for i in 0..8 {
            board[6][i] = Piece::White(Warrior::Pawn, 1);
        }

        return Self { board };
    }

    pub fn get_piece_at(&self, point: &Point) -> Piece {
        return self.board[point.x as usize][point.y as usize];
    }

    pub fn move_to(&mut self, piece: Piece, old_point: &Point, new_point: &Point) {
        self.board[old_point.x as usize][old_point.y as usize] = Piece::Empty;
        self.board[new_point.x as usize][new_point.y as usize] = piece;
    }
}
