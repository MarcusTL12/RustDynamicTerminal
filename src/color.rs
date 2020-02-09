
use std::io::Write;

#[allow(dead_code)]
pub enum Color {
    Def,
    Blink,
    White,
    LightCyan,
    Green,
    Hidden,
    Blue,
    LightMagenta,
    LightBlack,
    LightRed,
    LightYellow,
    Cyan,
    Bold,
    LightBlue,
    Nothing,
    Yellow,
    Magenta,
    Normal,
    Red,
    Black,
    Reverse,
    Underline,
    LightGreen,
}

pub fn getcolor(col: Color) -> &'static str {
    match col {
        Color::Def => "\x1b[39m",
        Color::Blink => "\x1b[5m",
        Color::White => "\x1b[37m",
        Color::LightCyan => "\x1b[96m",
        Color::Green => "\x1b[32m",
        Color::Hidden => "\x1b[8m",
        Color::Blue => "\x1b[34m",
        Color::LightMagenta => "\x1b[95m",
        Color::LightBlack => "\x1b[90m",
        Color::LightRed => "\x1b[91m",
        Color::LightYellow => "\x1b[93m",
        Color::Cyan => "\x1b[36m",
        Color::Bold => "\x1b[1m",
        Color::LightBlue => "\x1b[94m",
        Color::Nothing => "",
        Color::Yellow => "\x1b[33m",
        Color::Magenta => "\x1b[35m",
        Color::Normal => "\x1b[0m",
        Color::Red => "\x1b[31m",
        Color::Black => "\x1b[30m",
        Color::Reverse => "\x1b[7m",
        Color::Underline => "\x1b[4m",
        Color::LightGreen => "\x1b[92m"
    }
}

pub fn enablecolor(col: Color) {
    print!("{}", getcolor(col));
}

pub fn enablecolor_strm(strm: &mut Vec<u8>, col: Color) {
    write!(strm, "{}", getcolor(col)).unwrap();
}

pub fn disablecolor() {
    print!("{}", getcolor(Color::Def));
}

pub fn disablecolor_strm(strm: &mut Vec<u8>) {
    write!(strm, "{}", getcolor(Color::Def)).unwrap();
}
