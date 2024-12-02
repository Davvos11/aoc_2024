use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day01(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
    }


    format!("Part one: \t Part two: ")
}