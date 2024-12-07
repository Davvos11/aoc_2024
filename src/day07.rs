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
    let mut part2 = 0;

    for (result, parts) in equations {
        if operator_permutations(result, &parts, None, false) {
            part1 += result;
        }
        if operator_permutations(result, &parts, None, true) {
            part2 += result;
        }
    }


    format!("Part one: {part1}\t Part two: {part2}")
}

fn operator_permutations(goal: u64, parts: &[u64], current: Option<u64>, part2: bool) -> bool {
    if parts.is_empty() {
        return current.unwrap_or(0) == goal;
    }
    let mut parts = parts;
    let current = current.unwrap_or_else(|| {
        let c = parts[0];
        parts = &parts[1..];
        c
    });

    // Optimisation to abort early
    if current >= goal { return false; }

    let result =
        operator_permutations(goal, &parts[1..], Some(current + parts[0]), part2)
            || operator_permutations(goal, &parts[1..], Some(current * parts[0]), part2);
    if part2 {
        result || operator_permutations(goal, &parts[1..], Some(concat(current, parts[0])), part2)
    } else {
        result
    }
}

#[allow(unused)]
/// Used for me to get some intuition :)
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
    let choose_concat = operator_permutations_debug(goal, &parts[1..], format!("{current}{}", parts[0]));
    // Combine the three options
    choose_plus.into_iter().chain(choose_times).chain(choose_concat).collect()
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}
