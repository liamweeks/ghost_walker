use crate::prelude::*;

pub struct GameLogic;


impl GameLogic {
    pub fn get_possible_moves(&self, game: &Board, current_pos: &Point) -> Vec<Point> {
        let mut move_points = Vec::new();
        // println!("{:#?}", game);

        println!("Current Posiiton: {:#?}", &game.board[current_pos.y as usize][current_pos.x as usize].clone().unwrap_or_else(|| {Piece::Empty}));

        /* for x in 0..7 {
            for y in 0..7 {
                // println!("x: {:#?}         y: {:#?}", x_point, y_point)

                /* println!("{:#?}", &game.board[x][y])

                match &game.board[x][y] -> {
                    
                } */

                //println!("{:#?}", &game.board[x][y].clone().unwrap_or_else(|| {Piece::Empty}))
                println!("Current Posiiton: {:#?}", &game.board[current_pos.x as usize][current_pos.y as usize].clone().unwrap_or_else(|| {Piece::Empty}))


            }
         } */


    }
}