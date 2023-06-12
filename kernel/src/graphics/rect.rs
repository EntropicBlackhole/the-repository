use super::color::Color;
use crate::driver::framebuffer;

pub fn rect(x: usize, y: usize, w: usize, h: usize, color: &Color) -> () {
    for px in x..(w + x) {
        for py in y..(h + y) {
            framebuffer::set(px, py, color)
        }
    }
}
