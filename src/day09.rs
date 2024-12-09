use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use itertools::Itertools;

pub fn day09(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let input = line
        .chars().map(|c| c.to_digit(10).unwrap());

    let mut empty_space = false;
    let mut disk1: Vec<Option<usize>> = Vec::new();
    {
        let mut i = 0;
        for len in input {
            if empty_space {
                for _ in 0..len { disk1.push(None) }
            } else {
                for _ in 0..len { disk1.push(Some(i)) }
                i += 1;
            }
            empty_space = !empty_space;
        }
    }

    let mut disk2 = disk1.clone();

    while let Some(item) = disk1.pop() {
        if item.is_some() {
            if let Some(first_space) = disk1.iter().position(|x| x.is_none()) {
                disk1[first_space] = item;
            } else {
                // No empty spaces found anymore, put the popped item back at the end
                disk1.push(item);
                break;
            }
        }
    }

    let part1: usize = disk1.iter().enumerate()
        .map(|(i, id)| i * id.unwrap()).sum();

    let mut spaces = collect_empty_spaces(&disk2);
    // Iterate over non-empty items, starting at the back
    let mut current = 0;
    let mut length = 0;
    for item in disk2.clone().iter().rev()
        .filter_map(|item| item.map(|v| v)) {
        if item != current && length > 0 {
            let item_idx = disk2.iter().position(|x| x == &Some(current)).unwrap();
            // We found the end (or rather start) of the current item sequence
            // Now check if it fits somewhere
            if let Some(&empty_idx) = spaces.iter()
                .find(|(_, size)| *size >= length)
                .map(|(empty_idx, _)| empty_idx) {
                    // Move items there
                    for j in 0..length {
                        disk2[item_idx + j] = None;
                        disk2[empty_idx + j] = Some(current);
                    }
                    // Update lookup
                    spaces = collect_empty_spaces(&disk2[..item_idx]);
            }
        }
        if item != current {
            current = item;
            length = 0;
        }
        length += 1;
    }

    let part2: usize = disk2.iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|x| (i, x)))
        .map(|(i, id)| i * id).sum();

    format!("Part one: {part1}\t Part two: {part2}")
}

fn collect_empty_spaces(disk: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut candidate = None;
    let mut length = 0;
    for (i, item) in disk.iter().enumerate() {
        if item.is_some() {
            if let Some(idx) = candidate {
                result.push((idx, length));
                candidate = None;
            }
        } else {
            if candidate.is_none() {
                candidate = Some(i);
                length = 0;
            }
            length += 1;
        }
    }

    result
}

#[allow(unused)]
fn print_disk(disk: &[Option<usize>]) {
    println!("{}", disk.iter().map(|x| x.map(|x| x.to_string()).unwrap_or(".".to_string())).collect::<String>());
}