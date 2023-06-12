use core::char;

use crate::{driver::framebuffer, graphics::color::Color};

static BUFFER: &'static [u8] = include_bytes!("ASC16.bin");

const WIDTH: usize = 16;
const HEIGHT: usize = WIDTH / 2;

pub fn draw_char(c: char, x: usize, y: usize, color: &Color) {
    let offset: usize = ((c as usize) & 0xFF) * WIDTH;

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if (BUFFER[offset + i] & (0x80 >> j)) != 0 {
                framebuffer::graphics::set(x + j, y + i, color)
            }
        }
    }
}

pub fn draw_string(s: &str, x: usize, y: usize, color: &Color) {
    for (i, c) in s.chars().enumerate() {
        draw_char(c, x + (i * 8), y, color)
    }
}
