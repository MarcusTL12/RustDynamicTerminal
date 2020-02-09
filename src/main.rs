use std::io::Write;

mod color;
use color::Color;

fn main() {
    let mut w = Vec::new();
    color::enablecolor_strm(&mut w, Color::Red);
    writeln!(&mut w, "{}", "Hei").unwrap();
    color::disablecolor_strm(&mut w);
    writeln!(&mut w, "{}", "hade").unwrap();
    std::io::stdout().write_all(&w).unwrap();
}
