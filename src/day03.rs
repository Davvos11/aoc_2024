use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

    let part1: u64 = instructions.iter().map(|(a, b)| a * b).sum();

    format!("Part one: {part1} \t Part two: ")
}

fn parse(string: &str) -> (Option<(u64, u64)>, &str) {
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
                    return (Some((result1, result2)), &string[4 + i..]);
                }
            } else {
                return (None, &string[4 + i..]);
            }
        }
        (None, "")
    } else {
        (None, &string[1..])
    }
}