use std::collections::HashSet;
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
    let mut mases: HashSet<(usize, usize)> = HashSet::new();
    let mut part2 = 0;

    for round in 0..2 {

        // Horizontal (and reversed)
        for line in &grid {
            part1 += count_xmas(line.iter());
            part1 += count_xmas(line.iter().rev());
        }
        let mut new_mases = Vec::new();
        // Diagonals (start at 0,0 going down) (and reversed)
        for i in 0..grid.len() {
            let diagonal = grid.iter()
                .skip(i).enumerate()
                .filter_map(|(k, l)| l.get(k));
            part1 += count_xmas(diagonal.clone());
            part1 += count_xmas(diagonal.clone().rev());

            // Find MASes and get the coordinate of the A
            let len = diagonal.clone().count();
            new_mases.append(&mut find_mases(diagonal.clone()).iter()
                .map(|idx| (i + idx, *idx)).collect()
            );
            new_mases.append(&mut find_mases(diagonal.rev()).iter()
                .map(|idx| (i + len - idx - 1, len - idx - 1)).collect()
            );
        }
        // Diagonals (start at 1,0 going right) (and reversed)
        for j in 1..grid[0].len() {
            let diagonal = grid.iter()
                .enumerate()
                .filter_map(|(k, l)| { l.get(k + j) });
            part1 += count_xmas(diagonal.clone());
            part1 += count_xmas(diagonal.clone().rev());

            // Find MASes and get the coordinate of the A
            let len = diagonal.clone().count();
            new_mases.append(&mut find_mases(diagonal.clone()).iter()
                .map(|idx| (*idx, j + idx)).collect()
            );
            new_mases.append(&mut find_mases(diagonal.rev()).iter()
                .map(|idx| (len - idx - 1, j + len - idx - 1)).collect()
            );
        }

        if round == 0 {
            // First round: collect al found mases
            for &mas in &new_mases {
                mases.insert(mas);
            }
        } else {
            // Second round (after rotating): find mases with same A coord (transpose coord first)
            let rows = grid.len();
            for &mas in &new_mases {
                if mases.contains(&rotate_coord(mas, rows)) {
                    part2 += 1;
                }
            }
        }

        // Now rotate and do again
        let rows = grid.len();
        let cols = grid[0].len();
        grid = (0..cols).map(|col| {
            (0..rows).rev().map(|row| grid[row][col]).collect()
        }).collect();
    }

    format!("Part one: {part1}\t Part two: {part2}")
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

/// Returns the index (of the letter A) at which a MAS has been found
fn find_mases<'a>(line: impl Iterator<Item=&'a char>) -> Vec<usize> {
    // 1 = m, 2 = a, 0 = s
    let mut mases = Vec::new();
    let mut state = 0;
    for (i, &char) in line.enumerate() {
        if char == 'M' {
            state = 1
        } else if state == 1 && char == 'A' {
            state = 2
        } else if state == 2 && char == 'S' {
            state = 0;
            mases.push(i - 1);
        } else {
            state = 0
        }
    }

    mases
}

fn rotate_coord((y, x): (usize, usize), rows: usize) -> (usize, usize) {
    let y_new = rows - 1 - x;
    let x_new = y;
    (y_new, x_new)
}
