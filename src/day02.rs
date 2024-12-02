use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use itertools::Itertools;

pub fn day02(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut part1 = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split_whitespace();
        let numbers = split.map(|x| x.parse::<u64>().unwrap());
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
        if safe {
            part1 += 1;
        }
    }


    format!("Part one: {part1}\t Part two: ")
}