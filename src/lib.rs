pub mod color;
pub mod linereset;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use linereset::showandreset;
    use color::Color;
    #[test]
    fn test1() {
        let mut w = Vec::new();
        //
        color::enablecolor_strm(&mut w, Color::Red);
        writeln!(&mut w, "{}", "Hei").unwrap();
        color::disablecolor_strm(&mut w);
        writeln!(&mut w, "{}", "Hade").unwrap();
        showandreset(&mut w);
        //
        std::thread::sleep(std::time::Duration::from_secs(1));
        //
        writeln!(&mut w, "{}", "Hei").unwrap();
        color::enablecolor_strm(&mut w, Color::Red);
        writeln!(&mut w, "{}", "Hade").unwrap();
        color::disablecolor_strm(&mut w);
        showandreset(&mut w);
        //
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
