use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day01(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // Read columns into sorted sets
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse::<u64>().unwrap());
        list2.push(split.next().unwrap().parse::<u64>().unwrap());
    }

    list1.sort();
    list2.sort();

    let part1: u64 = list1.iter().zip(list2.iter()).map(|(&a, &b)| a.abs_diff(b)).sum();

    let mut part2 = 0;

    let counts2 = list2.iter()
        .fold(HashMap::new(), |mut m, x| {
            m.entry(x).and_modify(|x| *x += 1).or_insert(1);
            m
        });
    for number1 in list1 {
        part2 += number1 * counts2.get(&number1).unwrap_or(&0);
    }

    format!("Part one: {part1}\t Part two: {part2}")
}