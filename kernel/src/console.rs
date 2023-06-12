use crate::{driver::framebuffer, graphics::asc16};
use alloc::{fmt, string::ToString};

pub enum Colors {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    Yellow,
    White,
}

impl Colors {
    fn value(&self) -> &crate::graphics::color::Color {
        use crate::graphics::color;
        match *self {
            Colors::Black => color::BLACK,
            Colors::Blue => color::BLUE,
            Colors::Green => color::GREEN,
            Colors::Cyan => color::CYAN,
            Colors::Red => color::RED,
            Colors::Purple => color::PURPLE,
            Colors::Brown => color::BROWN,
            Colors::Gray => color::GRAY,
            Colors::DarkGray => color::DARK_GRAY,
            Colors::LightBlue => color::LIGHT_BLUE,
            Colors::LightGreen => color::LIGHT_GREEN,
            Colors::LightCyan => color::LIGHT_CYAN,
            Colors::Yellow => color::YELLOW,
            Colors::White => color::WHITE,
        }
    }
}

static mut SIZE: Option<[usize; 2]> = None;
static mut CURSOR: &'static mut [usize; 2] = &mut [0, 0];
static mut FOREGROUND: Colors = Colors::White;
static mut BACKGROUND: Colors = Colors::Black;

pub fn init() {
    clear();
    let info = framebuffer::info();
    let w = info.width / 8;
    let h = info.height / 16;
    set_size(w, h);
}
pub fn print(args: fmt::Arguments<'_>) {
    let binding = args.to_string();
    let string = match args.as_str() {
        Some(s) => s,
        None => &binding,
    };

    for c in string.chars() {
        printc(c);
    }
}
pub fn println(args: fmt::Arguments<'_>) {
    let binding = args.to_string();
    let string = match args.as_str() {
        Some(s) => s,
        None => &binding,
    };
    let mut string = string.to_string();
    string.push('\n');

    for c in string.chars() {
        printc(c);
    }
}

fn printc(c: char) {
    // if not ascii: return [OK]
    if !c.is_ascii() {
        return;
    }

    // if is newline: move to new line, return [OK]
    if c == '\n' {
        move_to_new_line();
        return;
    }

    // convert cursor pos to framebuffer pos [OK]
    let fp = unsafe { [CURSOR[0] * 8, CURSOR[1] * 16] };

    // fill with background [OK]
    framebuffer::graphics::rect(fp[0], fp[1], 8, 16, get_background());

    // draw char [OK]
    asc16::draw_char(c, fp[0], fp[1], get_foreground());

    // move cursor to left by 1 [ok]
    unsafe {
        CURSOR[0] += 1;
    }
    // if X is at the width, move to new line [OK]
    if unsafe { CURSOR[0] } == get_size()[0] {
        move_to_new_line();
    }
}

fn move_to_new_line() {
    unsafe {
        CURSOR[0] = 0;
        CURSOR[1] += 1;
    }
    try_scroll();
}

fn try_scroll() {
    if unsafe { CURSOR[1] } == get_size()[1] {
        unsafe {
            CURSOR[1] -= 1;
        }
        framebuffer::graphics::copy(
            0,
            16, // Start copying from one line below
            get_size()[0] * 8,
            (get_size()[1] - 1) * 16, // Copy all lines except the first one
            0,
            0, // Move the copied content up by one line
        );
        framebuffer::graphics::rect(
            0,
            get_size()[1] * 16 - 16, // Fill the last line
            get_size()[0] * 8,
            16,
            get_background(),
        );
    }
}

pub fn get_foreground() -> &'static crate::graphics::color::Color {
    unsafe { &FOREGROUND }.value()
}
pub fn set_foreground(value: Colors) {
    unsafe {
        FOREGROUND = value;
    }
}

pub fn get_background() -> &'static crate::graphics::color::Color {
    unsafe { &BACKGROUND }.value()
}
pub fn set_background(value: Colors) {
    unsafe {
        BACKGROUND = value;
    }
}

fn set_size(w: usize, h: usize) {
    unsafe { SIZE = Some([w, h]) }
}

fn get_size() -> [usize; 2] {
    unsafe { SIZE.expect("`SIZE` accessed before call to init") }
}

pub fn clear() {
    unsafe {
        CURSOR.fill(0);
    }
    framebuffer::graphics::clear(get_background());
}

#[macro_export]
macro_rules! println {
    () => (crate::console::println(format_args!("")));
    ($($arg:tt)*) => (crate::console::println(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::console::print(format_args!($($arg)*)));
}
