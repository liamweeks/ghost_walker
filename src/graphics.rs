use crate::prelude::*;


pub struct Graphics {
    pub buffer: Vec<u32>
}

impl Graphics {

    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

        return Self {
            buffer
        };
    }

    pub fn set_background(&mut self, colour: &u32) {
        for pixel in 0..self.buffer.len() {
            self.buffer[pixel] = *colour;
        }
    }

    pub fn colour_point(&mut self, point: Point, colour: u32) {

        let pixel = point.y * WIDTH as i32 + point.x;

        if pixel > (HEIGHT * WIDTH) as i32 || pixel < 0 {
            return // Since we don't want to draw out of the window boundaries
        } else {
            self.buffer[((point.x) as i32 + (WIDTH as i32 * (point.y) as i32)) as usize] = colour;
        }

    }

    pub fn draw_rect(&mut self, top_right_point: Point, dimensions: Point, colour: u32  ) {
        let offset_x = top_right_point.x;
        let offset_y = top_right_point.y;
        
        for length in 0..dimensions.x {

            for height in 0..dimensions.y {
                self.colour_point(Point::new(length + offset_x, height + offset_y), colour)
            }
        }
    
    }
    
}