use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day08(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let (part01, part02) = solve(&grid);

    format!("Part one: {part01}\t Part two: {part02}")
}

fn solve(grid: &[Vec<char>]) -> (usize, usize) {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let antennae_iter = grid.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate()
                .filter_map(move |(x, &c)| {
                    if c != '.' && c != '#' { Some((c, (y as isize, x as isize))) } else { None }
                })
        });
    
    let mut antennae = HashMap::new();
    for (c, coords) in antennae_iter {
        antennae.entry(c).or_insert(Vec::new()).push(coords);
    }

    let mut antinodes1 = HashSet::new();
    let mut antinodes2 = HashSet::new();

    for  coords in antennae.values() {
        for (i, &(y1, x1)) in coords.iter().enumerate() {
            for &(y2, x2) in coords[i + 1..].iter() {
                let distance = (y2 - y1, x2 - x1);
                // Part1: add antinodes at equal distance 
                let one = (y1 - distance.0, x1 - distance.1);
                let two = (y2 + distance.0, x2 + distance.1);
                if !coords.contains(&one) { antinodes1.insert(one); }
                if !coords.contains(&two) { antinodes1.insert(two); }
                // Part 2: keep adding antinodes at equal distances in line
                for antinode in get_coords(y1, x1, distance, rows, cols) {
                    antinodes2.insert(antinode);
                }
            }
        }
    }

    let part1 = antinodes1.iter()
        .filter(|(y, x)| *y >= 0 && *y < rows && *x >= 0 && *x < cols)
        .count();
    let part2 = antinodes2.len();

    (part1, part2)
}

fn get_coords(y0: isize, x0: isize, (dist_y, dist_x): (isize, isize), rows: isize, cols: isize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut y = y0;
    let mut x = x0;
    
    while y >= 0 && y < rows && x >= 0 && x < cols {
        result.push((y as usize, x as usize));
        y -= dist_y;
        x -= dist_x;
    }
    y = y0 + dist_y;
    x = x0 + dist_x;
    while y >= 0 && y < rows && x >= 0 && x < cols {
        result.push((y as usize, x as usize));
        y += dist_y;
        x += dist_x;
    }
    
    result
}