use std::io::Write;

pub fn showandreset(buff: &mut Vec<u8>) {
    std::io::stdout().lock().write_all(&buff).unwrap();
    let l = countlines(buff);
    buff.clear();
    write!(buff, "\x1b[999D\x1b[{}A", l).unwrap();
}

fn countlines(buff: &Vec<u8>) -> usize {
    buff.iter().filter(|&&x| x == 10).count()
}
