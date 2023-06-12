use super::{color_to_pixel, framebuffer, info, order_pixel};
use crate::graphics::color::Color;

pub fn rect(x: usize, y: usize, w: usize, h: usize, color: &Color) -> () {
    for px in x..(w + x) {
        for py in y..(h + y) {
            set(px, py, color)
        }
    }
}

pub fn clear(color: &Color) {
    let buf = framebuffer().buffer_mut();

    let info = info();
    let pixel = color_to_pixel(color);

    for offset in (0..buf.len()).step_by(info.bytes_per_pixel) {
        for (i, &v) in pixel.iter().enumerate() {
            buf[offset + i] = v;
        }
    }
}

pub fn set(x: usize, y: usize, color: &Color) {
    let info = info();
    let width = info.width;
    let height = info.height;

    if x >= width || y >= height {
        return;
    }

    let buf = framebuffer().buffer_mut();

    let pixel = color_to_pixel(color);
    let offset = (x + width * y) * info.bytes_per_pixel;

    for (i, &v) in pixel.iter().enumerate() {
        buf[offset + i] = v;
    }
}

pub fn get(x: usize, y: usize) -> Result<[u8; 4], ()> {
    let info = info();

    if x >= info.width || y >= info.height {
        return Err(());
    }

    let buf = framebuffer().buffer();

    let offset = (x + info.width * y) * info.bytes_per_pixel;
    let end_offset = offset + info.bytes_per_pixel;

    let pixel = &buf[offset..end_offset];
    let ordered = order_pixel(pixel);

    Ok(ordered)
}

pub fn copy(sx: usize, sy: usize, w: usize, h: usize, dx: usize, dy: usize) {
    let info = info();

    if sx >= info.width
        || sy >= info.height
        || dx >= info.width
        || dy >= info.height
        || w > info.width
        || h > info.width
    {
        return;
    }

    for x in 0..w {
        for y in 0..h {
            let color = get(x + sx, y + sy).expect("Out of bounds");
            let color = &Color::rgba(color[0], color[1], color[2], color[3]);
            set(x + dx, y + dy, color);
        }
    }
}
