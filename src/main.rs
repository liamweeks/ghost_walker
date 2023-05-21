use crate::prelude::*;

mod board;
mod piece;
mod point;

mod prelude {
    pub use crate::board::*;
    pub use crate::piece::*;
    pub use crate::point::*;
    pub use minifb::{Key, KeyRepeat, Window, WindowOptions};
    pub const SQUARE_SIZE: u32 = 80;
    pub const WIDTH: u32 = SQUARE_SIZE * 8;
    pub const HEIGHT: u32 = SQUARE_SIZE * 8;
}

fn main() {
    let game = Board::new();

    let mut window = Window::new(
        "Ghost Walker",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("{:#?}", game.board);

        match window.get_keys_pressed(KeyRepeat::No) {
            keys => {
                for key in &keys {
                    match keys {
                        _ => {}
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}
