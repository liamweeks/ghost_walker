use crate::prelude::*;

pub struct Graphics {
    pub buffer: Vec<u32>,
}

impl Graphics {
    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

        return Self { buffer };
    }

    pub fn colour_point(&mut self, point: Point, colour: u32) {
        let pixel = point.y * WIDTH as i32 + point.x;

        if pixel > (HEIGHT * WIDTH) as i32 || pixel < 0 {
            return; // Since we don't want to draw out of the window boundaries
        } else {
            self.buffer[((point.x) as i32 + (WIDTH as i32 * (point.y) as i32)) as usize] = colour;
        }
    }

    pub fn draw_rect(&mut self, top_right_point: Point, dimensions: &Point, colour: u32) {
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
                    Piece::White(warrior, _) => {
                        symbol = match warrior {
                            // White is uppercase
                            Warrior::King => String::from("K"),
                            Warrior::Pawn => String::from("P"),
                            Warrior::Knight => String::from("N"),
                            Warrior::Bishop => String::from("B"),
                            Warrior::Queen => String::from("Q"),
                            Warrior::Rook => String::from("R"),
                        }
                    }
                    Piece::Black(warrior, _) => {
                        symbol = match warrior {
                            // Black is lowercase
                            Warrior::King => String::from("k"),
                            Warrior::Pawn => String::from("p"),
                            Warrior::Knight => String::from("n"),
                            Warrior::Bishop => String::from("b"),
                            Warrior::Queen => String::from("q"),
                            Warrior::Rook => String::from("r"),
                        }
                    }
                    Piece::Empty => symbol = format!("{}{}", x, y),
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

                    text.change_colour(Colours::GREEN);

                     text.draw(
                        &mut self.buffer,
                        Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                        Colours::GREEN
                    );

/*                     Text::write_to_buffer(
                        &mut self.buffer,
                        SQUARE_SIZE * WIDTH,
                        &mut Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                        Colours::GREEN,
                        Colours::GREEN
                    ) */

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

            text.change_colour(Colours::BLUE);

              text.draw(
                        &mut self.buffer,
                        Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                        Colours::BLUE
                    ); 

/*                     Text::write_to_buffer(
                        &mut self.buffer,
                        SQUARE_SIZE * WIDTH,
                        &mut Point::new((x * SQUARE_SIZE + 35) as i32, (y * SQUARE_SIZE + 35) as i32),
                        &symbol,
                        Colours::GREEN,
                        Colours::GREEN
                    ) */
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
