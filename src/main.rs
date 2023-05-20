use minifb::{Key, Window, WindowOptions, KeyRepeat};


fn main() {

    let square_size = 40;
    let width = square_size * 8;
    let height = square_size * 8;

    let mut window = Window::new(
        "Ghost Walker",
        width,
        height,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| panic!("{}", e));


    let buffer: Vec<u32>  = vec![0; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {


        match window.get_keys_pressed(KeyRepeat::No) {
            
            keys => {
                for key in &keys {
                    match keys {
                      _ => {}
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
    };
}

