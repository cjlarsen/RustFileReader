use std::{env, vec};
#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn byte_size_for_unit(size: u64, unit: FileSize) -> u64 {
        match unit {
            FileSize::Bytes(_) => size,
            FileSize::Kilobytes(_) => size * 1000,
            FileSize::Megabytes(_) => size * 1_000_000,
            FileSize::Gigabytes(_) => size * 1_000_000_000,
        }
    }

    fn type_from_unit(unit: &str) -> FileSize {
        match unit {
            "kb" => FileSize::Kilobytes(0.0),
            "mb" => FileSize::Megabytes(0.0),
            "gb" => FileSize::Gigabytes(0.0),
            _ => FileSize::Bytes(0),
        }
    }
}

fn split_text(text: &str) -> Option<(u64, FileSize)> {
    let (num, unit) = text.split_at(text.len() - 2);

    let size = match num.trim().parse::<u64>() {
        Ok(size) => size,
        Err(_) => return None,
    };

    let unit = FileSize::type_from_unit(unit);

    return Some((size, unit));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size_text = match args.len() {
        2 => &args[1],
        _ => {
            eprintln!("Usage: {} <file size in bytes>", args[0]);
            std::process::exit(1);
        }
    };

    // let test_size_text = "10 gb";

    let (size, unit) = match split_text(size_text) {
        Some((size, unit)) => (size, unit),
        None => {
            eprintln!("Invalid file size format. Use <number> <unit> format. Example: 10 kb");
            std::process::exit(1);
        }
    };

    let size = FileSize::byte_size_for_unit(size, unit);

    let sizes: Vec<String> = vec![
        size.to_string(),
        (size as f64 / 1000.0).to_string(),
        (size as f64 / 1_000_000.0).to_string(),
        (size as f64 / 1_000_000_000.0).to_string(),
    ];

    println!("Sizes {{ bytes: {} bytes, kilobytes: {} kilobytes, megabytes: {} megabytes, gigabytes: {} gigabytes }}", sizes[0], sizes[1], sizes[2], sizes[3]);
}