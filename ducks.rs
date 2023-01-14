use std::env;
use std::fs;
use human_bytes::human_bytes;

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

        if path.is_dir() {
            let size = fs::read_dir(&path)
                .unwrap()
                .map(|res| res.unwrap())
                .filter_map(|res| {
                    let path = res.path();
                    if path.is_file() {
                        Some(fs::metadata(path).unwrap().len())
                    } else {
                        None
                    }
                })
                .sum();

            entries.push((path.to_str().unwrap().to_string(), size));
        } else if path.is_file() {
            let size = fs::metadata(&path).unwrap().len();
            entries.push((path.to_str().unwrap().to_string(), size));
        }
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    let max_path_len = entries
        .iter()
        .map(|entry| entry.0.len())
        .max()
        .unwrap_or(0);

    for (i, entry) in entries.iter().enumerate() {
        if i >= limit {
            break;
        }
        let size = human_bytes(entry.1 as f64);
        let path = &entry.0;
        print!("{:<width$}  ", path, width = max_path_len);
        let _ = std::io::stdout();
        println!("{}", size);
    }
}
