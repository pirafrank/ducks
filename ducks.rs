use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;
use human_bytes::human_bytes;
use rayon::prelude::*;

fn get_dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let size = if path.is_dir() {
            get_dir_size(&path)?
        } else if path.is_file() {
            fs::metadata(&path)?.len()
        } else {
            0
        };
        total_size += size;
    }
    Ok(total_size)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //  first arg is command path itself
    if args.len() < 2 {
        println!("Error: you need to provide a directory path as first argument.");
        return;
    }

    let dir = &args[1];
    let limit = args.get(2).unwrap_or(&String::from("10")).parse::<usize>().unwrap();

    let mut entries = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let size = if path.is_dir() {
            get_dir_size(&path)
        } else if path.is_file() {
            Ok(fs::metadata(&path).unwrap().len())
        } else {
            Ok(0)
        };

        entries.push((path.to_str().unwrap().to_string(), size));
    }

    // sort descending
    entries.sort_by(|a, b| {
        (b.1).as_ref().unwrap().cmp(&(a.1).as_ref().unwrap())
    });

    // find row with max length (column display)
    let max_path_len = entries
        .par_iter()
        .map(|entry| entry.0.len())
        .max()
        .unwrap_or(0);

    for (i, entry) in entries.iter().enumerate() {
        if i >= limit {
            break;
        }
        let size = human_bytes(*(entry.1).as_ref().unwrap() as f64);
        let path = &entry.0;
        print!("{:<width$}  ", path, width = max_path_len);
        let _ = std::io::stdout();
        println!("{}", size);
    }
}
