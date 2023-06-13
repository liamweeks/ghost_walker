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
    pub use crate::graphics::*;
    pub use crate::piece::*;
    pub use crate::point::*;
    pub use crate::text::*;
    pub use crate::game_state::*;
    pub use minifb::{Key, KeyRepeat, Window, WindowOptions};
    pub const SQUARE_SIZE: usize = 80;
    pub const WIDTH: usize = SQUARE_SIZE * 8;
    pub const HEIGHT: usize = SQUARE_SIZE * 8;
}

fn main() {
    let mut game = Board::new();
    let text = Text::new(WIDTH as usize, HEIGHT as usize, 2);
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

    let mut delta_x: i32 = 0;
    let mut delta_y: i32 = 0;

    let mut game_state = GameState::Player;
    
    let mut old_point = Point::new(0, 0);

    graphics.render_board(&game, &mouse, &Vec::new());
    while window.is_open() && !window.is_key_down(Key::Escape) {
        delta_x = 0;
        delta_y = 0;

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
                                    println!("{:#?}", possible_moves);
                                    graphics.render_board(&game, &mouse, &possible_moves);
                                    old_point = mouse;

                                    match window.get_keys_pressed(KeyRepeat::No) {
                                        keys => {
                                            for key in &keys {
                                                match key {
                                                    Key::M => {
                                                        for custom_move in &possible_moves {
                                                            if mouse.is_equivalent_to(&custom_move.destination) {
                                                                let piece = game.get_piece_at(&mouse);
                                                                game.clone().move_to(piece, &old_point.clone(), &mouse.clone());
                                                            }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                               
                                _ => {}
                            }
                        }
                    }
                }

                mouse.x += delta_x;
                mouse.y += delta_y;

                graphics.render_board(&game, &mouse, &possible_moves);

                //game_state = GameState::CPUThinking;
            }

            GameState::CPUThinking => {

                possible_moves = game_logic.get_possible_moves(&game, &mouse);

                game_state = GameState::CPUDisplay;
            }

            GameState::CPUDisplay => {

                game_state = GameState::Player;
            }
        }

        /* text.draw(
            &mut graphics.buffer,
            Point::new(20, (HEIGHT - 20) as i32),
            "Welcome to Ghost Walker",
        ); */

        window
            .update_with_buffer(&graphics.buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}
