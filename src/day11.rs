use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day11(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let input: Vec<u64> = reader.lines().next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse().unwrap()).collect();

    let part1: usize = input.iter()
        .map(|&num| solve_smart(num, 25, &mut HashMap::new()))
        .sum();
    let part2: usize = input.iter()
        .map(|&num| solve_smart(num, 75, &mut HashMap::new()))
        .sum();

    format!("Part one: {part1}\t Part two: {part2}")
}

fn solve_smart(number: u64, depth: u32, memo: &mut HashMap<(u64, u32), usize>) -> usize {
    if depth == 0 {
        return 1;
    }
    if let Some(&result) = memo.get(&(number, depth)) {
        return result;
    }

    let mut result = 0;
    let digits = digits(number);
    if number == 0 {
        result += solve_smart(1, depth - 1, memo);
    } else if digits % 2 == 0 {
        let (a, b) = split_num(number, digits / 2);
        result += solve_smart(a, depth - 1, memo)
            + solve_smart(b, depth - 1, memo);
    } else {
        result += solve_smart(number * 2024, depth - 1, memo)
    }

    memo.insert((number, depth), result);
    result
}

#[allow(unused)]
/// Used for part 1, too slow for part 2
fn solve_naive(input: &[u64], depth: u32) -> usize {
    let new = input.iter().fold(Vec::new(), |mut acc, &i| {
        let digits = digits(i);
        if i == 0 {
            acc.push(1);
        } else if digits % 2 == 0 {
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