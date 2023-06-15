use crate::prelude::*;
use crate::Piece::*;

pub struct GameLogic;

impl GameLogic {
    pub fn get_points_at(&self, game: &Board, pos: &Point) -> u8 {
        let piece: &Piece = &game.board[pos.y as usize][pos.x as usize];

        return match piece {
            White(_, points) => *points,
            Black(_, points) => *points,
            Empty => 0,
        };
    }

    pub fn consider_moves(&self, list_of_moves: Vec<CustomMove>) -> CustomMove {
        let mut best_possible_move = CustomMove::new(0, Point::new(0, 0));

        for possible_move in list_of_moves {
            if possible_move.points >= best_possible_move.points {
                best_possible_move = possible_move;
            }
        }

        return best_possible_move;
    }

    pub fn get_possible_moves(&self, game: &Board, current_pos: &Point) -> Vec<CustomMove> {
        let mut possible_moves: Vec<CustomMove> = Vec::new();
        println!(
            "Current Position: {:#?}",
            &game.board[current_pos.y as usize][current_pos.x as usize].clone()
        );

        let piece: &Piece = &game.board[current_pos.y as usize][current_pos.x as usize];
        let mut destination = Point::new(current_pos.x, current_pos.y);

        match piece {
            Piece::White(_, _) => {
                // Logic for white pieces
                match piece {
                    Piece::White(Warrior::Pawn, _) => {
                        // Logic for white pawn
                        // Add possible pawn moves to `possible_moves` vector

                        // Calculate possible moves for a white pawn
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Single move forward
                        if y > 0
                            && game.board[(y - 1).max(0) as usize][x as usize].clone()
                                == Piece::Empty
                        {
                            destination = Point {
                                x: current_pos.x,
                                y: current_pos.y - 1,
                            };

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }

                        // Double move forward from the starting position
                        if y == 6
                            && game.board[4][x as usize] == Piece::Empty
                            && game.board[5][x as usize] == Piece::Empty
                        {
                            destination = Point {
                                x: current_pos.x,
                                y: current_pos.y - 2,
                            };

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }

                        // Capture moves
                        if y > 0 {
                            // Capture to the left
                            if x > 0
                                && game.board[(y - 1).max(0) as usize][(x - 1).max(0) as usize]
                                    .is_black()
                            {
                                destination = Point::new(current_pos.x - 1, current_pos.y - 11);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            }

                            // Capture to the right
                            if x < 7
                                && game.board[(y - 1).max(0) as usize][(x + 1) as usize].is_black()
                            {
                                destination = Point::new(current_pos.x + 1, current_pos.y - 1);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            }
                        }
                    }
                    Piece::White(Warrior::Rook, _) => {
                        // Calculate possible moves for a white rook
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Vertical moves upwards
                        for i in (0..y).rev() {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_black() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Vertical moves downwards
                        for i in y + 1..8 {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_black() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the left
                        for i in (0..x).rev() {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_black() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the right
                        for i in x + 1..8 {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_black() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                    Piece::White(Warrior::Knight, _) => {
                        // Calculate possible moves for a white knight
                        let x = current_pos.x;
                        let y = current_pos.y;

                        let knight_moves = [
                            (1, 2),
                            (2, 1),
                            (-1, 2),
                            (-2, 1),
                            (1, -2),
                            (2, -1),
                            (-1, -2),
                            (-2, -1),
                        ];

                        for &(dx, dy) in &knight_moves {
                            let new_x = x as i32 + dx;
                            let new_y = y as i32 + dy;

                            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty()
                                    || game.board[new_y as usize][new_x as usize].is_black()
                                {
                                    destination = Point::new(new_x, new_y);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                }
                            }
                        }
                    }
                    Piece::White(Warrior::Bishop, _) => {
                        // Calculate possible moves for a white bishop
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Diagonal moves in four directions: top-left, top-right, bottom-left, bottom-right

                        // Top-left diagonal
                        let mut i = x as i32 - 1;
                        let mut j = y as i32 - 1;
                        while i >= 0 && j >= 0 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                            i -= 1;
                            j -= 1;
                        }

                        // Top-right diagonal
                        i = x as i32 + 1;
                        j = y as i32 - 1;
                        while i < 8 && j >= 0 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                            i += 1;
                            j -= 1;
                        }

                        // Bottom-left diagonal
                        i = x as i32 - 1;
                        j = y as i32 + 1;
                        while i >= 0 && j < 8 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                            i -= 1;
                            j += 1;
                        }

                        // Bottom-right diagonal
                        i = x as i32 + 1;
                        j = y as i32 + 1;
                        while i < 8 && j < 8 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);
                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);
                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                            i += 1;
                            j += 1;
                        }
                    }
                    Piece::White(Warrior::Queen, _) => {
                        // Calculate possible moves for a white queen
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Vertical moves upwards
                        for i in (0..y).rev() {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_black() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Vertical moves downwards
                        for i in y + 1..8 {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_black() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the left
                        for i in (0..x).rev() {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_black() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the right
                        for i in x + 1..8 {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_black() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Top-left diagonal
                        let mut i = x as i32 - 1;
                        let mut j = y as i32 - 1;
                        while i >= 0 && j >= 0 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                            i -= 1;
                            j -= 1;
                        }

                        // Top-right diagonal
                        i = x as i32 + 1;
                        j = y as i32 - 1;
                        while i < 8 && j >= 0 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                            i += 1;
                            j -= 1;
                        }

                        // Bottom-left diagonal
                        i = x as i32 - 1;
                        j = y as i32 + 1;
                        while i >= 0 && j < 8 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                            i -= 1;
                            j += 1;
                        }

                        // Bottom-right diagonal
                        i = x as i32 + 1;
                        j = y as i32 + 1;
                        while i < 8 && j < 8 {
                            if game.board[j as usize][i as usize].is_empty() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[j as usize][i as usize].is_black() {
                                destination = Point::new(i, j);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                            i += 1;
                            j += 1;
                        }
                    }
                    Piece::White(Warrior::King, _) => {
                        // Calculate possible moves for a white king
                        let x = current_pos.x as i32;
                        let y = current_pos.y as i32;

                        let king_moves = [
                            (x - 1, y - 1),
                            (x, y - 1),
                            (x + 1, y - 1),
                            (x - 1, y),
                            (x + 1, y),
                            (x - 1, y + 1),
                            (x, y + 1),
                            (x + 1, y + 1),
                        ];

                        for &(new_x, new_y) in &king_moves {
                            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty()
                                    || game.board[new_y as usize][new_x as usize].is_black()
                                {
                                    destination = Point::new(new_x, new_y);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                }
                            }
                        }
                    }

                    _ => {} // Empty square
                }
            }

            Piece::Black(_, _) => {
                // Logic for black pieces
                // Implement similar logic as above for each black piece type
                match piece {
                    Piece::Black(Warrior::Pawn, _) => {
                        // Calculate possible moves for a black pawn
                        let x = current_pos.x as i32;
                        let y = current_pos.y as i32;

                        // Single square forward
                        let forward = (x, y + 1);
                        if forward.1 < 8
                            && game.board[forward.1 as usize][forward.0 as usize].is_empty()
                        {
                            destination = Point::new(forward.0, forward.1);

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }

                        // Double square forward from starting position
                        let double_forward = (x, y + 2);
                        if y == 1
                            && game.board[double_forward.1 as usize][double_forward.0 as usize]
                                .is_empty()
                        {
                            destination = Point::new(double_forward.0, double_forward.1);

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }

                        // Capture moves
                        let capture_left = (x - 1, y + 1);
                        let capture_right = (x + 1, y + 1);

                        if capture_left.0 >= 0
                            && game.board[capture_left.1 as usize][capture_left.0 as usize]
                                .is_white()
                        {
                            destination = Point::new(capture_left.0, capture_left.1);

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }
                        if capture_right.0 < 8
                            && game.board[capture_right.1 as usize][capture_right.0 as usize]
                                .is_white()
                        {
                            destination = Point::new(capture_right.0, capture_right.1);

                            possible_moves.push(CustomMove {
                                destination,
                                points: self.get_points_at(game, &destination),
                            });
                        }
                    }
                    Piece::Black(Warrior::Rook, _) => {
                        // Calculate possible moves for a black rook
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Vertical moves upwards
                        for i in (0..y).rev() {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_white() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                                break;
                            } else {
                                break;
                            }
                        }

                        // Vertical moves downwards
                        for i in y + 1..8 {
                            if game.board[i as usize][x as usize].is_empty() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[i as usize][x as usize].is_white() {
                                destination = Point::new(current_pos.x, i as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the left
                        for i in (0..x).rev() {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_white() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the right
                        for i in x + 1..8 {
                            if game.board[y as usize][i as usize].is_empty() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][i as usize].is_white() {
                                destination = Point::new(i as i32, current_pos.y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }
                    }
                    Piece::Black(Warrior::Knight, _) => {
                        // Calculate possible moves for a black knight
                        let x = current_pos.x as i32;
                        let y = current_pos.y as i32;

                        let knight_moves = [
                            (x - 2, (y - 1).max(0)),
                            (x - 2, y + 1),
                            (x - 1, (y - 2).max(0)),
                            (x - 1, y + 2),
                            (x + 1, (y - 2).max(0)),
                            (x + 1, y + 2),
                            (x + 2, (y - 1).max(0)),
                            (x + 2, y + 1),
                        ];

                        for &(new_x, new_y) in &knight_moves {
                            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty()
                                    || game.board[new_y as usize][new_x as usize].is_white()
                                {
                                    destination = Point::new(new_x, new_y);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                }
                            }
                        }
                    }

                    Piece::Black(Warrior::Bishop, _) => {
                        // Calculate possible moves for a black bishop
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Diagonal moves towards the top-right
                        for i in 1..8 {
                            let new_x = x + i;
                            let new_y = y + i;
                            if new_x < 8 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the top-left
                        for i in 1..8 {
                            let new_x = x - i;
                            let new_y = y + i;
                            if new_x >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the bottom-left
                        for i in 1..8 {
                            let new_x = x - i;
                            let new_y = y - i;
                            if new_x >= 0 && new_y >= 0 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the bottom-right
                        for i in 1..8 {
                            let new_x = x + i;
                            let new_y = y - i;
                            if new_x < 8 && new_y >= 0 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    Piece::Black(Warrior::Queen, _) => {
                        // Calculate possible moves for a black queen
                        let x = current_pos.x;
                        let y = current_pos.y;

                        // Horizontal moves to the right
                        for new_x in (x + 1)..8 {
                            if game.board[y as usize][new_x as usize].is_empty() {
                                destination = Point::new(new_x as i32, y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][new_x as usize].is_white() {
                                destination = Point::new(new_x as i32, y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Horizontal moves to the left
                        for new_x in (0..x).rev() {
                            if game.board[y as usize][new_x as usize].is_empty() {
                                destination = Point::new(new_x as i32, y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[y as usize][new_x as usize].is_white() {
                                destination = Point::new(new_x as i32, y);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Vertical moves upwards
                        for new_y in (y + 1)..8 {
                            if game.board[new_y as usize][x as usize].is_empty() {
                                destination = Point::new(x, new_y as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[new_y as usize][x as usize].is_white() {
                                destination = Point::new(x, new_y as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Vertical moves downwards
                        for new_y in (0..y).rev() {
                            if game.board[new_y as usize][x as usize].is_empty() {
                                destination = Point::new(x, new_y as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });
                            } else if game.board[new_y as usize][x as usize].is_white() {
                                destination = Point::new(x, new_y as i32);

                                possible_moves.push(CustomMove {
                                    destination,
                                    points: self.get_points_at(game, &destination),
                                });

                                break;
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the top-right
                        for i in 1..8 {
                            let new_x = x + i;
                            let new_y = y + i;
                            if new_x < 8 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the top-left
                        for i in 1..8 {
                            let new_x = x - i;
                            let new_y = y + i;
                            if new_x >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the bottom-left
                        for i in 1..8 {
                            let new_x = x - i;
                            let new_y = y - i;
                            if new_x >= 0 && new_y >= 0 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        // Diagonal moves towards the bottom-right
                        for i in 1..8 {
                            let new_x = x + i;
                            let new_y = y - i;
                            if new_x < 8 && new_y >= 0 {
                                if game.board[new_y as usize][new_x as usize].is_empty() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                } else if game.board[new_y as usize][new_x as usize].is_white() {
                                    destination = Point::new(new_x as i32, new_y as i32);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });

                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    Piece::Black(Warrior::King, _) => {
                        // Calculate possible moves for a black king
                        let x = current_pos.x as i32;
                        let y = current_pos.y as i32;

                        let king_moves = [
                            (x - 1, y - 1),
                            (x - 1, y),
                            (x - 1, y + 1),
                            (x, y - 1),
                            (x, y + 1),
                            (x + 1, y - 1),
                            (x + 1, y),
                            (x + 1, y + 1),
                        ];

                        for &(new_x, new_y) in &king_moves {
                            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                                if game.board[new_y as usize][new_x as usize].is_empty()
                                    || game.board[new_y as usize][new_x as usize].is_white()
                                {
                                    destination = Point::new(new_x, new_y);

                                    possible_moves.push(CustomMove {
                                        destination,
                                        points: self.get_points_at(game, &destination),
                                    });
                                }
                            }
                        }
                    }
                    _ => {} // Empty square
                }
            }
            Piece::Empty => {} // Empty square
        }

        //possible_moves = self.sanitize_moves(&possible_moves);

        return possible_moves;
    }
}
