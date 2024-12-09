use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day09(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let input = line
        .chars().map(|c| c.to_digit(10).unwrap());

    let mut empty_space = false;
    let mut i = 0;
    let mut disk: Vec<Option<usize>> = Vec::new();
    for len in input {
        if empty_space {
            for _ in 0..len { disk.push(None) }
        } else {
            for _ in 0..len { disk.push(Some(i)) }
            i += 1;
        }
        empty_space = !empty_space;
    }

    while let Some(item) = disk.pop() {
        if item.is_some() {
            if let Some(first_space) = disk.iter().position(|x| x.is_none()) {
                disk[first_space] = item;
            } else {
                // No empty spaces found anymore, put the popped item back at the end
                disk.push(item);
                break;
            }
        }
    }

    let part1: usize = disk.iter().enumerate()
        .map(|(i, id)| i * id.unwrap()).sum();

    format!("Part one: {part1}\t Part two: ")
}

#[allow(unused)]
fn print_disk(disk: &[Option<usize>]) {
    println!("{}", disk.iter().map(|x| x.map(|x| x.to_string()).unwrap_or(".".to_string())).collect::<String>());
}