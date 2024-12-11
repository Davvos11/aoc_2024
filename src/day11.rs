use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day11(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let input: Vec<u64> = reader.lines().next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse().unwrap()).collect();

    let part1 = solve_naive(&input, 25);

    format!("Part one: {part1}\t Part two: ")
}

fn solve_naive(input: &[u64], depth: u32) -> usize {
    let new = input.iter().fold(Vec::new(), |mut acc, &i| {
        let digits = digits(i);
        if i == 0 { acc.push(1); } else if digits % 2 == 0 {
            let (a, b) = split_num(i, digits / 2);
            acc.push(a);
            acc.push(b);
        } else {
            acc.push(i * 2024);
        }
        acc
    });
    
    if depth > 1 {
        solve_naive(&new, depth - 1)
    } else {
        new.len()
    }
}

fn digits(n: u64) -> u32 {
    if n == 0 { 1 } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn split_num(n: u64, split: u32) -> (u64, u64) {
    let divisor = 10u64.pow(split);
    let first = n / divisor;
    let second = n % divisor;
    (first, second)
}