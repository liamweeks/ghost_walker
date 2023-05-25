use crate::prelude::*;

pub struct Text {
    texture: Vec<u32>,
    width: usize,
    //height: usize,
    scale: usize,
}

#[inline(always)]
pub fn color_from_bit(bit: u8) -> u32 {
    if bit == 0 {
        0x000000
        // *Colours::BLUE
    } else {
        // 0xFFFFFFFF
        *Colours::GREEN
    }
}

impl Text {
    pub fn new(width: usize, _height: usize, scale: usize) -> Self {
        // unpack texture for easier drawing
        let mut texture = Vec::with_capacity(128 * 128);

        for t in MICROKNIGHT_FONT {
            texture.push(color_from_bit((t >> 7) & 1));
            texture.push(color_from_bit((t >> 6) & 1));
            texture.push(color_from_bit((t >> 5) & 1));
            texture.push(color_from_bit((t >> 4) & 1));
            texture.push(color_from_bit((t >> 3) & 1));
            texture.push(color_from_bit((t >> 2) & 1));
            texture.push(color_from_bit((t >> 1) & 1));
            texture.push(color_from_bit(t & 1));
        }

        Self {
            texture,
            width,
            //height,
            scale,
        }
    }

    pub fn draw(&self, screen: &mut [u32], point: Point, text: &str) {
        let pos = point.to_usize();
        let mut x = pos.0;
        let y = pos.1;
        for c in text.chars() {
            let mut index = c as usize - ' ' as usize;
            if index > MICROKNIGHT_LAYOUT.len() as usize {
                index = 0;
            }

            let layout = MICROKNIGHT_LAYOUT[index];
            let texture_offset = (layout.1 as usize * 128) + layout.0 as usize;

            for fy in 0..8 * self.scale {
                let ty = fy / self.scale;
                for fx in 0..8 * self.scale {
                    let tx = fx / self.scale;
                    let pixel = texture_offset + (ty * 128) + tx;
                    if pixel != 0 {
                        screen[((y + fy) * self.width) + fx + x] = self.texture[pixel];
                    }
                }
            }

            x += 8 * self.scale;
        }
    }
}

// Microknight font (128x128 packed with 1 bit per pixel)
#[rustfmt::skip]
pub static MICROKNIGHT_FONT: &[u8] = &[
    0x00, 0x0c, 0x1b, 0x0d, 0x81, 0x03, 0x01, 0xc0, 0x30, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x0c, 0x1b, 0x0d, 0x87, 0xc4, 0xb3, 0x60, 0x30, 0x30, 0x0c, 0x1b, 0x03, 0x00, 0x00, 0x00,
    0x00, 0x0c, 0x09, 0x1f, 0xcd, 0x03, 0xe1, 0xc0, 0x10, 0x60, 0x06, 0x0e, 0x03, 0x00, 0x00, 0x00,
    0x00, 0x0c, 0x00, 0x0d, 0x87, 0xc0, 0xc3, 0xd8, 0x20, 0x60, 0x06, 0x3f, 0x8f, 0xc0, 0x03, 0xe0,
    0x00, 0x0c, 0x00, 0x1f, 0xc1, 0x61, 0x83, 0x70, 0x00, 0x60, 0x06, 0x0e, 0x03, 0x01, 0x80, 0x00,
    0x00, 0x00, 0x00, 0x0d, 0x81, 0x63, 0x63, 0x60, 0x00, 0x30, 0x0c, 0x1b, 0x03, 0x01, 0x80, 0x00,
    0x00, 0x0c, 0x00, 0x0d, 0x87, 0xc6, 0x91, 0xf0, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x80, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x60, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x03, 0x07, 0xc1, 0xe0, 0x61, 0xf0, 0x70, 0x7f, 0x1e, 0x0f, 0x00, 0x00, 0x00,
    0x00, 0x03, 0x1e, 0x03, 0x00, 0x60, 0x30, 0x61, 0x80, 0xc0, 0x03, 0x33, 0x19, 0x81, 0x80, 0xc0,
    0x00, 0x06, 0x33, 0x07, 0x03, 0xc0, 0xe0, 0xc1, 0xf8, 0xfc, 0x06, 0x1f, 0x18, 0xc1, 0x80, 0xc0,
    0x00, 0x0c, 0x37, 0x83, 0x06, 0x00, 0x31, 0xb0, 0x0c, 0xc6, 0x0c, 0x31, 0x98, 0xc0, 0x00, 0x00,
    0x00, 0x18, 0x3d, 0x83, 0x0c, 0x02, 0x33, 0x30, 0x8c, 0xc6, 0x0c, 0x31, 0x8f, 0xc0, 0x00, 0x00,
    0x18, 0x30, 0x39, 0x83, 0x0c, 0x06, 0x33, 0xf9, 0x98, 0xcc, 0x0c, 0x33, 0x00, 0xc1, 0x80, 0xc0,
    0x18, 0x60, 0x1f, 0x0f, 0xcf, 0xe3, 0xe0, 0x30, 0xf0, 0x78, 0x0c, 0x1e, 0x03, 0x81, 0x80, 0x40,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x0f, 0x83, 0x83, 0xc3, 0xe0, 0xf0, 0xf8, 0x7f, 0x3f, 0x87, 0x0c, 0x63, 0xf0,
    0x18, 0x00, 0x0c, 0x18, 0xc6, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x60, 0x30, 0x0c, 0x0c, 0x60, 0xc0,
    0x30, 0x3e, 0x06, 0x00, 0xcd, 0xe6, 0x33, 0xf1, 0x80, 0xc6, 0x7e, 0x3f, 0x18, 0x0c, 0x60, 0xc0,
    0x60, 0x00, 0x03, 0x07, 0x8f, 0x67, 0xf3, 0x19, 0x80, 0xc6, 0x60, 0x30, 0x19, 0xcf, 0xe0, 0xc0,
    0x30, 0x3e, 0x06, 0x06, 0x0d, 0xe6, 0x33, 0x19, 0x80, 0xc6, 0x60, 0x30, 0x18, 0xcc, 0x60, 0xc0,
    0x18, 0x00, 0x0c, 0x00, 0x0c, 0x06, 0x33, 0x31, 0x8c, 0xc6, 0x60, 0x30, 0x18, 0xcc, 0x60, 0xc0,
    0x00, 0x00, 0x00, 0x06, 0x06, 0x66, 0x33, 0xe0, 0xf8, 0xfc, 0x7f, 0x30, 0x0f, 0xcc, 0x63, 0xf0,
    0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0e, 0x63, 0x30, 0x18, 0xcc, 0x63, 0xc3, 0xe0, 0xf0, 0xf8, 0x3c, 0x1f, 0x98, 0xcc, 0x66, 0x30,
    0x06, 0x66, 0x30, 0x1d, 0xce, 0x66, 0x63, 0x31, 0x98, 0xcc, 0x60, 0x06, 0x18, 0xcc, 0x66, 0x30,
    0x06, 0x6c, 0x30, 0x1f, 0xcf, 0x66, 0x33, 0x19, 0x8c, 0xc6, 0x3e, 0x06, 0x18, 0xcc, 0x66, 0x30,
    0x06, 0x78, 0x30, 0x1a, 0xcd, 0xe6, 0x33, 0x19, 0x8c, 0xc6, 0x03, 0x06, 0x18, 0xc6, 0xc6, 0xb0,
    0xc6, 0x6c, 0x30, 0x18, 0xcc, 0xe6, 0x33, 0xf1, 0x8c, 0xfc, 0x23, 0x06, 0x18, 0xc6, 0xc7, 0xf0,
    0xc6, 0x66, 0x30, 0x18, 0xcc, 0x66, 0x33, 0x01, 0xac, 0xd8, 0x63, 0x06, 0x18, 0xc3, 0x87, 0x70,
    0x7c, 0x63, 0x3f, 0x98, 0xcc, 0x63, 0xe3, 0x00, 0xf8, 0xcc, 0x3e, 0x06, 0x0f, 0x83, 0x86, 0x30,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xc6, 0x63, 0x3f, 0x87, 0x00, 0x01, 0xc0, 0x40, 0x00, 0x18, 0x00, 0x30, 0x00, 0x00, 0x60, 0x00,
    0x6c, 0x63, 0x03, 0x06, 0x0c, 0x00, 0xc0, 0xe0, 0x00, 0x18, 0x1e, 0x3e, 0x0f, 0x03, 0xe3, 0xc0,
    0x38, 0x63, 0x06, 0x06, 0x06, 0x00, 0xc1, 0xb0, 0x00, 0x10, 0x03, 0x33, 0x19, 0x86, 0x66, 0x60,
    0x38, 0x3e, 0x0c, 0x06, 0x03, 0x00, 0xc0, 0x00, 0x00, 0x08, 0x3f, 0x31, 0x98, 0x0c, 0x67, 0xe0,
    0x6c, 0x06, 0x18, 0x06, 0x01, 0x80, 0xc0, 0x00, 0x00, 0x00, 0x63, 0x31, 0x98, 0x0c, 0x66, 0x00,
    0xc6, 0x06, 0x30, 0x06, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0x00, 0x63, 0x31, 0x98, 0xcc, 0x66, 0x30,
    0xc6, 0x06, 0x3f, 0x87, 0x00, 0x61, 0xc0, 0x00, 0x00, 0x00, 0x3f, 0x3f, 0x0f, 0x87, 0xe3, 0xe0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x38, 0x00, 0x30, 0x03, 0x00, 0xc6, 0x00, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x6c, 0x3f, 0x3e, 0x00, 0x00, 0x06, 0x60, 0x61, 0x88, 0xf8, 0x3c, 0x3e, 0x07, 0xcf, 0xc3, 0xc0,
    0x60, 0x63, 0x33, 0x07, 0x01, 0xc6, 0xc0, 0x61, 0xdc, 0xcc, 0x66, 0x33, 0x0c, 0xcc, 0x66, 0x00,
    0x78, 0x63, 0x31, 0x83, 0x00, 0xc7, 0x80, 0x61, 0xfc, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x03, 0xe0,
    0x60, 0x63, 0x31, 0x83, 0x00, 0xc6, 0xc0, 0x61, 0xac, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x00, 0x30,
    0x60, 0x3f, 0x31, 0x83, 0x00, 0xc6, 0x60, 0x61, 0x8c, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x06, 0x30,
    0x60, 0x03, 0x31, 0x8f, 0xc4, 0xc6, 0x31, 0xf9, 0x8c, 0xc6, 0x3e, 0x3f, 0x0f, 0xcc, 0x03, 0xe0,
    0x60, 0x3e, 0x00, 0x00, 0x03, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0xc0, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x18, 0x1c, 0x87, 0x00, 0x00, 0xc0,
    0x7c, 0x63, 0x31, 0x98, 0xcc, 0x66, 0x33, 0xf8, 0x30, 0x18, 0x0c, 0x27, 0x0e, 0x00, 0x00, 0x00,
    0x30, 0x63, 0x31, 0x9a, 0xc6, 0xc6, 0x30, 0x30, 0x30, 0x18, 0x0c, 0x00, 0x1c, 0x00, 0x00, 0xc0,
    0x30, 0x63, 0x1b, 0x1f, 0xc3, 0x86, 0x30, 0x60, 0x60, 0x18, 0x06, 0x00, 0x18, 0x20, 0x00, 0xc0,
    0x30, 0x63, 0x1b, 0x0f, 0x83, 0x86, 0x30, 0xc0, 0x30, 0x18, 0x0c, 0x00, 0x10, 0x60, 0x00, 0xc0,
    0x32, 0x63, 0x0e, 0x0d, 0x86, 0xc3, 0xf1, 0x80, 0x30, 0x18, 0x0c, 0x00, 0x00, 0xe0, 0x00, 0xc0,
    0x1c, 0x3f, 0x0e, 0x08, 0x8c, 0x60, 0x33, 0xf8, 0x18, 0x18, 0x18, 0x00, 0x01, 0xc0, 0x00, 0xc0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xe0, 0x00, 0x00, 0x18, 0x00, 0x00, 0x03, 0x80, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x30, 0x1c, 0x00, 0x18, 0xc1, 0x83, 0xc1, 0xb0, 0x78, 0x00, 0x00, 0x1f, 0x00, 0x03, 0xc3, 0xe0,
    0x78, 0x36, 0x31, 0x98, 0xc1, 0x86, 0x00, 0x00, 0x84, 0x7e, 0x1b, 0x03, 0x00, 0x04, 0x20, 0x00,
    0xcc, 0x30, 0x1f, 0x18, 0xc1, 0x83, 0xe0, 0x01, 0x32, 0xc6, 0x36, 0x00, 0x00, 0x0b, 0x90, 0x00,
    0xc0, 0x7c, 0x31, 0x8f, 0x80, 0x06, 0x30, 0x01, 0x42, 0xc6, 0x6c, 0x00, 0x0f, 0x8a, 0x50, 0x00,
    0xc0, 0x30, 0x31, 0x81, 0x81, 0x86, 0x30, 0x01, 0x42, 0x7e, 0x36, 0x00, 0x00, 0x0b, 0x90, 0x00,
    0xc6, 0x30, 0x1f, 0x07, 0xc1, 0x83, 0xe0, 0x01, 0x32, 0x00, 0x1b, 0x00, 0x00, 0x0a, 0x50, 0x00,
    0x7c, 0x7f, 0x31, 0x81, 0x81, 0x80, 0x30, 0x00, 0x84, 0x7c, 0x00, 0x00, 0x00, 0x04, 0x20, 0x00,
    0x30, 0x00, 0x00, 0x00, 0x00, 0x01, 0xe0, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x38, 0x00, 0x1c, 0x0e, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x06, 0x03, 0x00,
    0x6c, 0x08, 0x06, 0x03, 0x03, 0x06, 0x31, 0xf8, 0x00, 0x00, 0x38, 0x1f, 0x1b, 0x0e, 0x67, 0x30,
    0x6c, 0x3e, 0x0c, 0x06, 0x06, 0x06, 0x33, 0xd0, 0x30, 0x00, 0x18, 0x31, 0x8d, 0x86, 0xc3, 0x60,
    0x38, 0x08, 0x18, 0x03, 0x00, 0x06, 0x31, 0xd0, 0x30, 0x00, 0x18, 0x31, 0x86, 0xc7, 0xa3, 0xc0,
    0x00, 0x00, 0x1e, 0x0e, 0x00, 0x06, 0x30, 0x50, 0x00, 0x00, 0x3c, 0x1f, 0x0d, 0x83, 0x61, 0xf0,
    0x00, 0x3e, 0x00, 0x00, 0x00, 0x06, 0x30, 0x50, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x06, 0xf3, 0x18,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xe0, 0x50, 0x00, 0x18, 0x00, 0x1f, 0x00, 0x0c, 0xf6, 0x70,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x30, 0xf8,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xe0, 0x18, 0x0c, 0x03, 0x03, 0x03, 0x91, 0xb0, 0xf0, 0x3f, 0x3c, 0x0c, 0x03, 0x01, 0x83, 0x60,
    0x36, 0x00, 0x02, 0x04, 0x0c, 0xc4, 0xe0, 0x01, 0x98, 0x6c, 0x66, 0x02, 0x04, 0x06, 0x60, 0x00,
    0x6c, 0x18, 0x1e, 0x0f, 0x07, 0x83, 0xc1, 0xe0, 0xf0, 0xcf, 0x60, 0x3f, 0x9f, 0xcf, 0xe7, 0xf0,
    0x3a, 0x1e, 0x33, 0x19, 0x8c, 0xc6, 0x63, 0x31, 0x98, 0xfc, 0x60, 0x30, 0x18, 0x0c, 0x06, 0x00,
    0xf6, 0x03, 0x3f, 0x9f, 0xcf, 0xe7, 0xf3, 0xf9, 0xfc, 0xcc, 0x60, 0x3f, 0x1f, 0x8f, 0xc7, 0xe0,
    0x6f, 0x63, 0x31, 0x98, 0xcc, 0x66, 0x33, 0x19, 0x8c, 0xcc, 0x63, 0x30, 0x18, 0x0c, 0x06, 0x00,
    0xcf, 0x3e, 0x31, 0x98, 0xcc, 0x66, 0x33, 0x19, 0x8c, 0xcf, 0x3e, 0x3f, 0x9f, 0xcf, 0xe7, 0xf0,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x30, 0x0c, 0x06, 0x0d, 0x8f, 0x83, 0x90, 0xc0, 0x30, 0x30, 0x39, 0x1b, 0x00, 0x07, 0x81, 0x80,
    0x08, 0x10, 0x19, 0x80, 0x0c, 0xc4, 0xe0, 0x20, 0x40, 0xcc, 0x4e, 0x00, 0x0f, 0x8c, 0xc0, 0x40,
    0x7e, 0x3f, 0x1f, 0x8f, 0xcc, 0x67, 0x31, 0xe0, 0xf0, 0x78, 0x3c, 0x1e, 0x1a, 0xcd, 0xe6, 0x30,
    0x18, 0x0c, 0x06, 0x03, 0x0e, 0x67, 0xb3, 0x31, 0x98, 0xcc, 0x66, 0x33, 0x1f, 0xef, 0x66, 0x30,
    0x18, 0x0c, 0x06, 0x03, 0x0c, 0x66, 0xf3, 0x19, 0x8c, 0xc6, 0x63, 0x31, 0x9b, 0x6e, 0x66, 0x30,
    0x18, 0x0c, 0x06, 0x03, 0x0c, 0x66, 0x73, 0x19, 0x8c, 0xc6, 0x63, 0x31, 0x98, 0xec, 0x66, 0x30,
    0x7e, 0x3f, 0x1f, 0x8f, 0xcf, 0xc6, 0x31, 0xf0, 0xf8, 0x7c, 0x3e, 0x1f, 0x0f, 0xc7, 0xc3, 0xe0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x18, 0x0c, 0x1b, 0x03, 0x0c, 0x00, 0x00, 0xc0, 0x30, 0x18, 0x39, 0x1b, 0x07, 0x80, 0x00, 0x00,
    0x20, 0x33, 0x00, 0x04, 0x0f, 0x83, 0xc0, 0x20, 0x40, 0x66, 0x4e, 0x00, 0x0c, 0xc7, 0xe3, 0xc0,
    0xc6, 0x63, 0x31, 0x98, 0xcc, 0xc6, 0x60, 0xf0, 0x78, 0x3c, 0x1e, 0x0f, 0x07, 0x81, 0xb6, 0x60,
    0xc6, 0x63, 0x31, 0x98, 0xcc, 0x66, 0xe0, 0x18, 0x0c, 0x06, 0x03, 0x01, 0x80, 0xc7, 0xf6, 0x00,
    0xc6, 0x63, 0x31, 0x8f, 0x8f, 0xc6, 0x31, 0xf8, 0xfc, 0x7e, 0x3f, 0x1f, 0x8f, 0xcd, 0x86, 0x00,
    0xc6, 0x63, 0x31, 0x81, 0x8c, 0x06, 0x33, 0x19, 0x8c, 0xc6, 0x63, 0x31, 0x98, 0xcd, 0x86, 0x30,
    0x7c, 0x3e, 0x1f, 0x01, 0x8c, 0x06, 0xe1, 0xf8, 0xfc, 0x7e, 0x3f, 0x1f, 0x8f, 0xc7, 0xf3, 0xe0,
    0x00, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x30, 0x0c, 0x0c, 0x0d, 0x83, 0x00, 0xc0, 0x60, 0xd8, 0x0c, 0x39, 0x0c, 0x03, 0x01, 0x83, 0x90,
    0x08, 0x10, 0x33, 0x00, 0x00, 0x81, 0x01, 0x98, 0x00, 0x16, 0x4e, 0x02, 0x04, 0x06, 0x64, 0xe0,
    0x78, 0x3c, 0x1e, 0x0f, 0x03, 0x81, 0xc0, 0xe0, 0x70, 0x3e, 0x7c, 0x1e, 0x0f, 0x07, 0x83, 0xc0,
    0xfc, 0x7e, 0x3f, 0x1f, 0x81, 0x80, 0xc0, 0x60, 0x30, 0x66, 0x66, 0x33, 0x19, 0x8c, 0xc6, 0x60,
    0xc0, 0x60, 0x30, 0x18, 0x01, 0x80, 0xc0, 0x60, 0x30, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x66, 0x30,
    0xc6, 0x63, 0x31, 0x98, 0xc1, 0x80, 0xc0, 0x60, 0x30, 0xc6, 0x63, 0x31, 0x98, 0xcc, 0x66, 0x30,
    0x7c, 0x3e, 0x1f, 0x0f, 0x87, 0xe3, 0xf1, 0xf8, 0xfc, 0x7e, 0x63, 0x1f, 0x0f, 0x87, 0xc3, 0xe0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x6c, 0x00, 0x00, 0x06, 0x01, 0x80, 0xc1, 0xb0, 0x30, 0xc0, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x0c, 0x1e, 0x01, 0x02, 0x03, 0x30, 0x00, 0x40, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x78, 0x00, 0x33, 0x18, 0xcc, 0x66, 0x33, 0x19, 0x8c, 0xf8, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xcc, 0x3f, 0x37, 0x98, 0xcc, 0x66, 0x33, 0x19, 0x8c, 0xcc, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xc6, 0x00, 0x3d, 0x98, 0xcc, 0x66, 0x33, 0x19, 0x8c, 0xc6, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xc6, 0x0c, 0x39, 0x98, 0xcc, 0x66, 0x33, 0x18, 0xfc, 0xfc, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x7c, 0x00, 0x1f, 0x0f, 0xc7, 0xe3, 0xf1, 0xf8, 0x0c, 0xc0, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xc0, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// Font layout (generated from Angelcode Bitmap Font generator)
#[rustfmt::skip]
pub static MICROKNIGHT_LAYOUT: &[(u8, u8)] = &[
    (0, 0), (9, 0), (18, 0), (27, 0), (36, 0), (45, 0), (54, 0), (63, 0), (72, 0), (81, 0), (90, 0), (99, 0), (108, 0),
    (117, 0), (0, 9), (9, 9), (18, 9), (27, 9), (36, 9), (45, 9), (54, 9), (63, 9), (72, 9), (81, 9), (90, 9), (99, 9),
    (108, 9), (117, 9), (0, 18), (9, 18), (18, 18), (27, 18), (36, 18), (45, 18), (54, 18), (63, 18), (72, 18), (81, 18),
    (90, 18), (99, 18), (108, 18), (117, 18), (0, 27), (9, 27), (18, 27), (27, 27), (36, 27), (45, 27), (54, 27), (63, 27),
    (72, 27), (81, 27), (90, 27), (99, 27), (108, 27), (117, 27), (0, 36), (9, 36), (18, 36), (27, 36), (36, 36), (45, 36),
    (54, 36), (63, 36), (72, 36), (81, 36), (90, 36), (99, 36), (108, 36), (117, 36), (0, 45), (9, 45), (18, 45), (27, 45),
    (36, 45), (45, 45), (54, 45), (63, 45), (72, 45), (81, 45), (90, 45), (99, 45), (108, 45), (117, 45), (0, 54), (9, 54),
    (18, 54), (27, 54), (36, 54), (45, 54), (54, 54), (63, 54), (72, 54), (81, 54), (90, 54), (99, 54), (0, 0), (0, 0),
    (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
    (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
    (108, 54), (117, 54), (0, 63), (9, 63), (18, 63), (27, 63), (36, 63), (45, 63), (54, 63), (63, 63), (72, 63), (81, 63),
    (90, 63), (99, 63), (108, 63), (117, 63), (0, 72), (9, 72), (18, 72), (27, 72), (36, 72), (45, 72), (54, 72), (63, 72),
    (72, 72), (81, 72), (90, 72), (99, 72), (108, 72), (117, 72), (0, 81), (9, 81), (18, 81), (27, 81), (36, 81), (45, 81),
    (54, 81), (63, 81), (72, 81), (81, 81), (90, 81), (99, 81), (108, 81), (117, 81), (0, 90), (9, 90), (18, 90), (27, 90),
    (36, 90), (45, 90), (54, 90), (63, 90), (72, 90), (81, 90), (90, 90), (99, 90), (108, 90), (117, 90), (0, 99), (9, 99),
    (18, 99), (27, 99), (36, 99), (45, 99), (54, 99), (63, 99), (72, 99), (81, 99), (90, 99), (99, 99), (108, 99), (117, 99),
    (0, 108), (9, 108), (18, 108), (27, 108), (36, 108), (45, 108), (54, 108), (63, 108), (72, 108), (81, 108), (90, 108),
    (99, 108), (108, 108), (117, 108), (0, 117), (9, 117), (18, 117), (27, 117), (36, 117), (45, 117), (54, 117), (63, 117),
    (72, 117), (81, 117),
];
