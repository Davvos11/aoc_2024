use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day04(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line: Vec<_> = line.unwrap()
            .chars().collect();
        grid.push(line);
    }

    let mut part1 = 0;

    for _ in 0..2 {
        // Horizontal (and reversed)
        for line in &grid {
            part1 += count_xmas(line.iter());
            part1 += count_xmas(line.iter().rev());
        }
        // Diagonals (start at 0,0 going down) (and reversed)
        for i in 0..grid.len() {
            let diagonal = grid.iter()
                .skip(i).enumerate()
                .filter_map(|(k, l)| l.get(k));
            part1 += count_xmas(diagonal.clone());
            part1 += count_xmas(diagonal.rev());
        }
        // Diagonals (start at 1,0 going right) (and reversed)
        for j in 1..grid[0].len() {
            let diagonal = grid.iter()
                .enumerate()
                .filter_map(|(k, l)| { l.get(k + j) });
            part1 += count_xmas(diagonal.clone());
            part1 += count_xmas(diagonal.rev());
        }

        // Now rotate and do again
        let rows = grid.len();
        let cols = grid[0].len();
        grid = (0..cols).map(|col| {
            (0..rows).rev().map(|row| grid[row][col]).collect()
        }).collect();
    }

    format!("Part one: {part1}\t Part two: ")
}

fn count_xmas<'a>(line: impl Iterator<Item=&'a char>) -> u32 {
    // 1 = x, 2 = m, 3 = a, 0 = s
    let mut count = 0;
    let mut state = 0;
    for &char in line {
        if char == 'X' {
            state = 1
        } else if state == 1 && char == 'M' {
            state = 2
        } else if state == 2 && char == 'A' {
            state = 3
        } else if state == 3 && char == 'S' {
            state = 0;
            count += 1;
        } else {
            state = 0
        }
    }

    count
}