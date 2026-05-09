use std::{env, thread};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::process::exit;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("사용법: tail <파일경로>");
        exit(1);
    }

    let file_dir = args.get(1).unwrap();
    let mut file = File::open(Path::new(file_dir)).unwrap();
    let reader = BufReader::new(&mut file);

    let lines: Vec<String> = reader.lines()
        .map(|l| l.unwrap())
        .collect();

    let start = if lines.len() > 10 {
        lines.len() - 10
    } else {
        0
    };

    for line in &lines[start..] {
        println!("{}", line);
    }

    let mut pos = file.seek(SeekFrom::End(0)).unwrap();
    loop {
        thread::sleep(Duration::from_millis(500));

        let new_pos = file.seek(SeekFrom::End(0)).unwrap();
        if new_pos > pos {
            file.seek(SeekFrom::Start(pos)).unwrap();
            let reader = BufReader::new(&mut file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
            pos = new_pos;
        }
    }
}