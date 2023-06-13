use crate::prelude::*;

pub struct Graphics {
    pub buffer: Vec<u32>,
}

impl Graphics {
    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

        return Self { buffer };
    }

    pub fn set_background(&mut self, colour: &u32) {
        for pixel in 0..self.buffer.len() {
            self.buffer[pixel] = *colour;
        }
    }

    pub fn colour_point(&mut self, point: Point, colour: &u32) {
        let pixel = point.y * WIDTH as i32 + point.x;

        if pixel > (HEIGHT * WIDTH) as i32 || pixel < 0 {
            return; // Since we don't want to draw out of the window boundaries
        } else {
            self.buffer[((point.x) as i32 + (WIDTH as i32 * (point.y) as i32)) as usize] = *colour;
        }
    }

    pub fn draw_rect(&mut self, top_right_point: Point, dimensions: &Point, colour: &u32) {
        let offset_x = top_right_point.x;
        let offset_y = top_right_point.y;

        for length in 0..dimensions.x {
            for height in 0..dimensions.y {
                self.colour_point(Point::new(length + offset_x, height + offset_y), colour)
            }
        }
    }

    pub fn render_board(&mut self, game: &Board, mouse: &Point, possible_moves: &Vec<CustomMove>) {
        let square = Point::new(SQUARE_SIZE as i32, SQUARE_SIZE as i32);
        let text = Text::new(WIDTH as usize, HEIGHT as usize, 2);

        for y in 0..8 {
            let offset = y % 2;

            for x in 0..8 {
                let current_piece = &game.board[y][x];

                let symbol;

                match current_piece {
                    Piece::White(warrior, points) => {
                        symbol = match warrior {
                            Warrior::King => String::from("Ki(W)"),
                            Warrior::Pawn => String::from("P(W)"),
                            Warrior::Knight => String::from("Kn(W)"),
                            Warrior::Bishop => String::from("Bi(W)"),
                            Warrior::Queen => String::from("Qu(W)"),
                            Warrior::Rook => String::from("Rk(W)")

                        }
                    }
                    Piece::Black(warrior, points) => {
                        symbol = match warrior {
                            Warrior::King => String::from("Ki(B)"),
                            Warrior::Pawn => String::from("P(B)"),
                            Warrior::Knight => String::from("Kn(B)"),
                            Warrior::Bishop => String::from("Bi(B)"),
                            Warrior::Queen => String::from("Qu(B)"),
                            Warrior::Rook => String::from("Rk(B)")

                        }
                    }
                    Piece::Empty => symbol = String::from(""),
                }

                for custom_move in possible_moves {
                    //println!("possible move found at ({}, {})", x, y);
                    self.draw_rect(
                        Point::new(
                            custom_move.destination.x * SQUARE_SIZE as i32,
                            custom_move.destination.y * SQUARE_SIZE as i32,
                        ),
                        &square,
                        Colours::GRAY,
                    );
                }

                if (x + offset) % 2 == 0 {
                    self.draw_rect(
                        Point::new((x * SQUARE_SIZE) as i32, (y * SQUARE_SIZE) as i32),
                        &square,
                        Colours::WHITE,
                    );

                    if &Point::new(x as i32, y as i32) == mouse {
                        self.draw_rect(
                            Point::new((x * SQUARE_SIZE) as i32, (y * SQUARE_SIZE) as i32),
                            &square,
                            Colours::RED,
                        );
                    }

                    text.draw(
                        &mut self.buffer,
                        Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                    );
                } else {
                    self.draw_rect(
                        Point::new((x * SQUARE_SIZE) as i32, (y * SQUARE_SIZE) as i32),
                        &square,
                        Colours::BLACK,
                    );

                    if &Point::new(x as i32, y as i32) == mouse {
                        self.draw_rect(
                            Point::new((x * SQUARE_SIZE) as i32, (y * SQUARE_SIZE) as i32),
                            &square,
                            Colours::RED,
                        );
                    }

                    text.draw(
                        &mut self.buffer,
                        Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                    );
                }

                if &Point::new(x as i32, y as i32) == mouse {
                    self.draw_rect(
                        Point::new((x * SQUARE_SIZE) as i32, (y * SQUARE_SIZE) as i32),
                        &square,
                        Colours::RED,
                    );
                }
            }
        }
    }
}
