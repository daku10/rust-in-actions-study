use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    // パターン引数を抽出する
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input");
    match input {
        Some(val) => {
            let f = File::open(val).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }
    }
}
