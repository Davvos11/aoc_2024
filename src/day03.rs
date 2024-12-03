use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::day03::Token::{Do, Dont, Mul};

pub fn day03(input: &PathBuf) -> String {
    let mut file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Can't read file");

    let mut parse_string = &input[..];
    let mut instructions = Vec::new();
    while !parse_string.is_empty() {
        let (result, tail) = parse(parse_string);
        if let Some(instruction) = result {
            instructions.push(instruction);
        }
        parse_string = tail;
    }

    // Part 1: just sum all multiplications
    let part1: u64 = instructions.iter()
        .filter_map(|t| if let Mul(a, b) = t { Some((a, b)) } else { None })
        .map(|(a, b)| a * b).sum();
    
    // Part 2: sum multiplications if enabled. Do or Dont toggle enabled
    // abuse a fold to do this :)
    let (_, part2) = instructions.iter()
        .fold((true, 0), |(enabled, sum), t| {
            match t {
                Mul(a, b) => {
                    if enabled { (enabled, sum + a * b) } else { (enabled, sum) }
                }
                Do => { (true, sum) }
                Dont => { (false, sum) }
            }
        });

    format!("Part one: {part1} \t Part two: {part2}")
}

enum Token {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse(string: &str) -> (Option<Token>, &str) {
    let mut num1 = Vec::new();
    let mut num2 = Vec::new();
    let mut first = true;
    if string.starts_with("mul(") {
        for (i, char) in string[4..].chars().enumerate() {
            if char.is_numeric() {
                if first {
                    num1.push(char);
                } else {
                    num2.push(char);
                }
            } else if char == ',' {
                if first {
                    first = false;
                } else {
                    return (None, &string[4 + i..]);
                }
            } else if char == ')' {
                if first {
                    return (None, &string[4 + i..]);
                } else {
                    let result1 = num1.into_iter().collect::<String>().parse().unwrap();
                    let result2 = num2.into_iter().collect::<String>().parse().unwrap();
                    return (Some(Mul(result1, result2)), &string[4 + i..]);
                }
            } else {
                return (None, &string[4 + i..]);
            }
        }
        (None, "")
    } else if string.starts_with("do()") {
        (Some(Do), &string[4..])
    } else if string.starts_with("don't()") {
        (Some(Dont), &string[7..])
    } else {
        (None, &string[1..])
    }
}