use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day07(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(": ");
        let result = split.next().unwrap().parse().unwrap();
        let parts = split.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        equations.push((result, parts))
    }

    let mut part1 = 0;

    for (result, parts) in equations {
        let possible = operator_permutations(result, &parts, None);
        // println!("{result}: {:?}, {:?}", operator_permutations_debug(result, &parts, String::new()), possible );
        if possible { part1 += result }
    }


    format!("Part one: {part1}\t Part two: ")
}

fn operator_permutations(goal: u64, parts: &[u64], current: Option<u64>) -> bool {
    if parts.is_empty() {
        return current.unwrap_or(0) == goal;
    }
    let mut parts = parts;
    let current = current.unwrap_or_else(|| {
        let c = parts[0];
        parts = &parts[1..];
        c
    });
    let choose_plus = operator_permutations(goal, &parts[1..], Some(current + parts[0]));
    let choose_times = operator_permutations(goal, &parts[1..], Some(current * parts[0]));
    // Combine the two options
    choose_plus || choose_times
}

fn operator_permutations_debug(goal: u64, parts: &[u64], current: String) -> Vec<String> {
    if parts.is_empty() {
        return vec![current];
    }
    let mut current = current;
    let mut parts = parts;
    if current.is_empty() {
        current = parts[0].to_string();
        parts = &parts[1..];
    }
    let choose_plus = operator_permutations_debug(goal, &parts[1..], format!("{current} + {}", parts[0]));
    let choose_times = operator_permutations_debug(goal, &parts[1..], format!("{current} * {}", parts[0]));
    // Combine the two options
    choose_plus.into_iter().chain(choose_times).collect()
}