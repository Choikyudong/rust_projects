use std::fs;
use chrono::{DateTime, Local};

fn main() {
    let entries = fs::read_dir(".").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let meta_data = entry.metadata().unwrap();

        let dir: &str = if meta_data.is_dir() {
            "d"
        } else {
            "-"
        };

        let read_only = meta_data.permissions().readonly();

        let file_size: String = if meta_data.is_dir() {
            "-".to_string()
        } else {
            meta_data.len().to_string()
        };

        let modified: DateTime<Local> = meta_data.modified().unwrap().into();

        let formatted = modified.format("%b %d %H:%M").to_string();

        println!("{} {} {} {} {}", dir, read_only, formatted, file_size, entry.file_name().to_string_lossy());
    }
}