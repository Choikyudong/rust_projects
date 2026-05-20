use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: grep <file> <pattern>");
        std::process::exit(-1);
    }

    let file = &args[1];
    let pattern = &args[2];

    let file = File::open(file)
        .expect("파일을 확인해주세요.");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}