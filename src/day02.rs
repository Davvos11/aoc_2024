use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day02(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split_whitespace();
        let numbers = split.map(|x| x.parse::<u64>().unwrap());
        if is_safe(numbers.clone()) {
            part1 += 1;
        }
        let numbers: Vec<_> = numbers.collect();
        for i in 0..numbers.len() {
            let mut numbers_ = numbers.clone();
            numbers_.remove(i);
            if is_safe(numbers_.into_iter()) {
                part2 += 1;
                break;
            }
        }
    }

    format!("Part one: {part1}\t Part two: {part2}")
}

fn is_safe(numbers: impl Iterator<Item=u64>) -> bool {
    let mut safe = true;
    let mut increasing = None;
    for (a, b) in numbers.tuple_windows::<(u64, u64)>() {
        if a.abs_diff(b) < 1 || a.abs_diff(b) > 3 {
            safe = false;
            break;
        }
        if let Some(increasing) = increasing {
            if (increasing && a > b) || (!increasing && a < b) {
                safe = false;
                break;
            }
        } else {
            increasing = Some(a < b)
        }
    }
    safe
}