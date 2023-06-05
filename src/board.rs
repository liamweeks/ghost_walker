use crate::prelude::*;

pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board = [[None; 8]; 8];

        // Populate the board with board
        // Assuming a standard starting position for a chess game
        // Pawns
        for file in 0..8 {
            board[1][file] = Some(Piece::new(1, "Pawn".to_string(), Team::White));
            board[6][file] = Some(Piece::new(1, "Pawn".to_string(), Team::Black));
        }

        // Rooks
        board[0][0] = Some(Piece::new(5, "Rook".to_string(), Team::White));
        board[0][7] = Some(Piece::new(5, "Rook".to_string(), Team::White));
        board[7][0] = Some(Piece::new(5, "Rook".to_string(), Team::Black));
        board[7][7] = Some(Piece::new(5, "Rook".to_string(), Team::Black));

        // Knights
        board[0][1] = Some(Piece::new(3, "Knight".to_string(), Team::White));
        board[0][6] = Some(Piece::new(3, "Knight".to_string(), Team::White));
        board[7][1] = Some(Piece::new(3, "Knight".to_string(), Team::Black));
        board[7][6] = Some(Piece::new(3, "Knight".to_string(), Team::Black));

        // Bishops
        board[0][2] = Some(Piece::new(3, "Bishop".to_string(), Team::White));
        board[0][5] = Some(Piece::new(3, "Bishop".to_string(), Team::White));
        board[7][2] = Some(Piece::new(3, "Bishop".to_string(), Team::Black));
        board[7][5] = Some(Piece::new(3, "Bishop".to_string(), Team::Black));

        // Queens
        board[0][3] = Some(Piece::new(9, "Queen".to_string(), Team::White));
        board[7][3] = Some(Piece::new(9, "Queen".to_string(), Team::Black));

        // Kings
        board[0][4] = Some(Piece::new(0, "King".to_string(), Team::White));
        board[7][4] = Some(Piece::new(0, "King".to_string(), Team::Black));

        Board { board }
    }

    /// GAME LOGIC

    pub fn get_possible_moves(&self, current_pos: &Point) {


        for row in &self.board {
            for element in row {
               match element {
                Some(Piece) => println!("I hit a pawn"),
                _ => println!("I hit something, but I don't know what"),
                None => {}
               }
            }
        }

    }
}
