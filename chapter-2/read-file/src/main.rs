use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);
    // 微妙に更新。BufReader::lines()は各業の最後にある改行を除く
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
