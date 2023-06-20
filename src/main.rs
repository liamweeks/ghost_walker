use crate::prelude::*;

mod board;
mod colours;
mod custom_move;
mod game_logic;
mod game_state;
mod graphics;
mod piece;
mod point;
mod text;

mod prelude {
    pub use crate::board::*;
    pub use crate::colours::*;
    pub use crate::custom_move::*;
    pub use crate::game_logic::GameLogic;
    pub use crate::game_state::*;
    pub use crate::graphics::*;
    pub use crate::piece::*;
    pub use crate::point::*;
    pub use crate::text::*;
    pub use minifb::{Key, KeyRepeat, Window, WindowOptions};
    pub const SQUARE_SIZE: usize = 80;
    pub const WIDTH: usize = SQUARE_SIZE * 8;
    pub const HEIGHT: usize = SQUARE_SIZE * 8;
}

use std::io;
use std::io::prelude::*;

fn main() {
    let mut game = Board::new();
    let game_logic = GameLogic;

    let mut window = Window::new(
        "Ghost Walker",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    let mut graphics = Graphics::new();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut mouse = Point::new(4, 4);
    let mut possible_moves = game_logic.get_possible_moves(&game, &mouse);


    let mut game_state = GameState::Player;

    graphics.render_board(&game, &mouse, &Vec::new());
    while window.is_open() && !window.is_key_down(Key::Escape) {


        //graphics.set_background(Colours::BLUE);

        match game_state {
            GameState::Player => {
                match window.get_keys_pressed(KeyRepeat::No) {
                    keys => {
                        for key in &keys {
                            match key {
                                Key::A => {
                                    if mouse.x > 0 {
                                        mouse.x -= 1;
                                    }
                                    println!("{:?}", &mouse);
                                }
                                Key::D => {
                                    if mouse.x < 7 {
                                        mouse.x += 1;
                                    }
                                    println!("{:?}", &mouse);
                                }
                                Key::S => {
                                    if mouse.y < 7 {
                                        mouse.y += 1;
                                    }
                                    println!("{:?}", &mouse);
                                }
                                Key::W => {
                                    if mouse.y > 0 {
                                        mouse.y -= 1;
                                    }
                                    println!("{:?}", &mouse);
                                }
                                Key::K => {
                                    possible_moves = game_logic.get_possible_moves(&game, &mouse);
                                    graphics.render_board(&game, &mouse, &possible_moves);
                                }
                                Key::M => {
                                    possible_moves = game_logic.get_possible_moves(&game, &mouse);
                                    println!("You presssed M");

                                    let x_coord: i32 = input(String::from("X")).parse().unwrap();
                                    let y_coord: i32 = input(String::from("Y")).parse().unwrap();

                                    game.move_to(&mouse, &Point::new(x_coord, y_coord));
                                    graphics.render_board(&game, &mouse, &possible_moves);
                                }

                                _ => {}
                            }
                        }
                    }
                }


                graphics.render_board(&game, &mouse, &possible_moves);

            }

            GameState::CPUThinking => {
                possible_moves = game_logic.get_possible_moves(&game, &mouse);

                game_state = GameState::CPUDisplay;
            }

            GameState::CPUDisplay => {
                game_state = GameState::Player;
            }
        }

        window
            .update_with_buffer(&graphics.buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}

fn input(query: String) -> String {
    print!("{}: ", query);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    return input;
}
