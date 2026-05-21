use std::{env, fs};
use std::path::Path;
use chrono::{DateTime, Local};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: stat <file>");
        std::process::exit(-1);
    }

    let meta_data = fs::metadata(&args[1]).unwrap();
    let path = Path::new(&args[1]);
    println!("fileName: {}", path.file_name().unwrap().to_string_lossy());
    println!("fileSize: {}", meta_data.len().to_string());

    let created: DateTime<Local> = meta_data.created().unwrap().into();
    let formatted = created.format("%b %d %H:%M").to_string();
    println!("created: {:#?}", formatted);

    let modified: DateTime<Local> = meta_data.modified().unwrap().into();
    let formatted = modified.format("%b %d %H:%M").to_string();
    println!("modified: {:?}", formatted);
}