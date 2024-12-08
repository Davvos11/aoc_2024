use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use itertools::Itertools;

pub fn day08(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let part01 = part01(&grid);

    format!("Part one: {part01}\t Part two: ")
}

fn part01(grid: &[Vec<char>]) -> usize {
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

    // println!("{antennae:?}");

    let mut antinodes = HashSet::new();

    for (c, coords) in antennae {
        // Loop through all possible pairs
        for (i, &(y1, x1)) in coords.iter().enumerate() {
            // println!("{coords:?}");
            for &(y2, x2) in coords[i + 1..].iter() {
                let distance = (y2 - y1, x2 - x1);
                if y1 + distance.0 == y2 && x1 + distance.1 == x2 {
                    let one = (y1 - distance.0, x1 - distance.1);
                    let two = (y2 + distance.0, x2 + distance.1);
                    if !coords.contains(&one) {
                        antinodes.insert(one);
                    }
                    if !coords.contains(&two) {
                        antinodes.insert(two);
                    }
                }
            }
        }
    }

    // println!("{:?}", antinodes.iter().sorted().map(|(y,x)| (y, x)));

    antinodes.iter()
        .filter(|(y, x)| *y >= 0 && *y < rows && *x >= 0 && *x < cols)
        .count()
}
