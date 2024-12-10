use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day10(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<u32>> = reader.lines()
        .map(|l| {
            l.unwrap().chars()
                // Parse '.' as impossibly high number so the examples work
                .map(|c| if c == '.' { 99 } else { c.to_digit(10).unwrap() })
                .collect()
        })
        .collect();

    let mut map = Map::new(&grid);
    let mut part1 = 0;
    // For each 0 tile
    for (y, x) in grid.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter(|&(_, &h)| h == 0).map(move |(x, _)| (y, x))
        }) {
        let score = map.get_trail_ends(y, x, 0);
        // println!("{},{}: {}\t{:?}", y+1, (b'a' + x as u8) as char, score.len(), score.iter().map(|(y,x)| (y+1, (b'a' + *x as u8) as char)).collect_vec());
        part1 += score.len();
        map.reset_visited();
    }

    format!("Part one: {part1}\t Part two: ")
}

struct Map<'a> {
    grid: &'a [Vec<u32>],
    visited: HashSet<(usize, usize)>,
}

impl<'a> Map<'a> {
    pub fn new(grid: &'a [Vec<u32>]) -> Self {
        Self {
            grid,
            visited: HashSet::new(),
        }
    }
    pub fn reset_visited(&mut self) {
        self.visited = HashSet::new();
    }

    pub fn get_trail_ends(&mut self, y: usize, x: usize, current_h: u32) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        self.visited.insert((y, x));

        // Check all directions for a one higher number
        for (d_y, d_x) in DIRS {
            if let (Some(y_new), Some(x_new)) = (y.checked_add_signed(d_y), x.checked_add_signed(d_x)) {
                // Don't check if already visiting during another recursion branch
                if self.visited.contains(&(y_new, x_new)) {
                    continue;
                }
                if let Some(&h) = self.grid.get(y_new).and_then(|row| row.get(x_new)) {
                    if h == current_h + 1 {
                        // Otherwise calculate and recurse
                        if h == 9 {
                            result.insert((y_new, x_new));
                        } else {
                            result.extend(self.get_trail_ends(y_new, x_new, h));
                        }
                        // println!("{y_new} {x_new} = {h}: {result}");
                    }
                }
            }
        }

        result
    }
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];