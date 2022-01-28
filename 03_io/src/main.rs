use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::ops::Deref;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(String::deref)
        .unwrap_or("/dev/stdin");

    let file = File::open(filename).unwrap_or_else(|_| {
        println!("Could not open file '{}'", filename);
        exit(1)
    });
    let file = file;
    let mut idx = 0;


    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            idx = idx + 1;
            println!("{}| {}", idx, line);
        }
    }
}
