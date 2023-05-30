use crate::prelude::*;

mod board;
mod colours;
mod graphics;
mod piece;
mod point;
mod text;

mod prelude {
    pub use crate::board::*;
    pub use crate::colours::*;
    pub use crate::graphics::*;
    pub use crate::piece::*;
    pub use crate::point::*;
    pub use crate::text::*;
    pub use minifb::{Key, KeyRepeat, Window, WindowOptions};
    pub const SQUARE_SIZE: usize = 80;
    pub const WIDTH: usize = SQUARE_SIZE * 8;
    pub const HEIGHT: usize = SQUARE_SIZE * 8;
}

fn main() {
    let game = Board::new();
    let text = Text::new(WIDTH as usize, HEIGHT as usize, 2);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut delta_x: i32 = 0;
        let mut delta_y: i32 = 0;


        graphics.set_background(Colours::BLUE);
        graphics.render_board(&game, &mouse);

        
        

        match window.get_keys_pressed(KeyRepeat::No) {
            keys => {
                for key in &keys {
                    match key {
                        Key::A => {
                             delta_x = -1;
                        },
                        Key::D => {
                            delta_x = 1;
                        }
                        Key::S => {
                            delta_y = 1;
                        }

                        Key::W => {
                            delta_y = -1;
                        }
                        _ => {}
                    }
                }
            }
        }

        mouse.x += delta_x;
        mouse.y += delta_y;

        graphics.render_board(&game, &mouse);

        text.draw(
            &mut graphics.buffer,
            Point::new(20, (HEIGHT - 20) as i32),
            "Welcome to Ghost Walker",
        );

        window
            .update_with_buffer(&graphics.buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();

           
    }
}
