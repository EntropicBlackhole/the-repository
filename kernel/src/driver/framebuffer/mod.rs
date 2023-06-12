use crate::graphics::color::Color;
use bootloader_api::info::PixelFormat;

pub mod graphics;

static mut BL_FRAMEBUFFER: Option<&'static mut bootloader_api::info::FrameBuffer> = None;

pub fn init(fb: &'static mut bootloader_api::info::FrameBuffer) {
    unsafe {
        BL_FRAMEBUFFER = Some(fb);
    }
}

fn color_to_pixel(color: &Color) -> [u8; 4] {
    let info = info();

    return match info.pixel_format {
        PixelFormat::Bgr => [color.b, color.g, color.r, color.a],
        PixelFormat::Rgb => [color.r, color.b, color.g, color.a],
        // PixelFormat::U8 => [color.grayscale(), color.grayscale()],
        PixelFormat::Unknown {
            red_position,
            green_position,
            blue_position,
        } => {
            let mut pixel = [0; 4];
            pixel[red_position as usize] = color.r;
            pixel[green_position as usize] = color.g;
            pixel[blue_position as usize] = color.b;
            pixel
        }
        _ => [color.r, color.b, color.g, color.a],
    };
}

/// Orders a pixel to `r, g, b, a` to support multiple pixel formats
fn order_pixel(pixel: &[u8]) -> [u8; 4] {
    let info = info();

    return match info.pixel_format {
        PixelFormat::Bgr => [pixel[2], pixel[0], pixel[1], pixel[3]],
        PixelFormat::Rgb => [pixel[0], pixel[1], pixel[2], pixel[3]],
        PixelFormat::Unknown {
            red_position,
            green_position,
            blue_position,
        } => {
            let mut ordered = [0; 4];
            ordered[red_position as usize] = pixel[0];
            ordered[green_position as usize] = pixel[1];
            ordered[blue_position as usize] = pixel[2];
            ordered
        }
        _ => [pixel[0], pixel[1], pixel[2], pixel[3]],
    };
}

pub fn framebuffer() -> &'static mut bootloader_api::info::FrameBuffer {
    unsafe {
        match &mut BL_FRAMEBUFFER {
            Some(ref mut x) => *x,
            &mut None => panic!("`BL_FRAMEBUFFER` accessed before call to init"),
        }
    }
}

pub fn info() -> bootloader_api::info::FrameBufferInfo {
    framebuffer().info()
}
