use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}
