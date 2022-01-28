use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filenames = match args.len() {
        0 => vec!["/dev/stdin"],
        _ => args.iter().map(String::deref).collect(),
    };

    let mut idx = 0;

    for filename in filenames {
        let file = File::open(filename).unwrap_or_else(|_| {
            println!("Could not open file '{}'", filename);
            exit(1)
        });

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                idx = idx + 1;
                println!("{}| {}", idx, line);
            }
        }
    }
}
