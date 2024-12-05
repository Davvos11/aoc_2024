use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use itertools::Itertools;

pub fn day05(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut first = true;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut lists: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if first {
            if line.is_empty() {
                first = false;
                continue;
            }
            let mut parsed = line.split("|").map(|s| s.parse().unwrap());
            if let (Some(a), Some(b)) = (parsed.next(), parsed.next()) {
                rules.push((a, b));
            }
        } else {
            lists.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for list in lists {
        let mut correct = true;
        for (rule_a, rule_b) in &rules {
            if let Some(pos_a) = list.iter().position(|x| x == rule_a) {
                if let Some(pos_b) = list.iter().position(|x| x == rule_b) {
                    if pos_a > pos_b {
                        correct = false;
                        break;
                    }
                }
            }
        }

        if correct {
            part1 += list[list.len() / 2]
        } else {
            let mut sorted = list.iter().sorted_by(|&&a, &&b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            part2 += sorted.nth(list.len() / 2).unwrap();
        }
    }


    format!("Part one: {part1}\t Part two: {part2}")
}