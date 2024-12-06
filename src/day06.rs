use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn day06(input: &PathBuf) -> String {
    let file = File::open(input).expect(&format!("Can't open file {:?}", input));
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    // (y,x)
    let mut position = (0, 0);
    // -1,0: up, 1,0: down, 0,-1: left, 0,1: right
    let mut direction = (-1, 0);

    for (y, line) in reader.lines().enumerate() {
        let line: Vec<_> = line.unwrap().chars().collect();
        if let Some(x) = line.iter().position(|&x| x == '^') {
            position = (y, x);
        }
        grid.push(line);
    }
    
    let rows = grid.len();
    let cols = grid[0].len();

    let mut positions = HashSet::new();
    positions.insert(position);

    while let Some(next_pos) = get_next_obstacle(&grid, direction, position) {
        for intermediate_pos in positions_between(position, next_pos, direction) {
            positions.insert(intermediate_pos);
        }
        position = next_pos;
        direction = next_direction(direction);
    }
    let next_pos = final_position(position, direction, rows, cols);
    for intermediate_pos in positions_between(position, next_pos, direction) {
        positions.insert(intermediate_pos);
    }


    format!("Part one: {}\t Part two: ", positions.len())
}

fn get_next_obstacle(grid: &[Vec<char>], dir: (i32, i32), pos: (usize, usize)) -> Option<(usize, usize)> {
    let column = grid.iter().map(|r| r[pos.1]).enumerate();
    let row = grid[pos.0].iter().enumerate();
    match dir {
        (-1, 0) => {
            // Get all cells above and find first obstacle
            if let Some((y, _)) = column
                .filter(|&(y, _)| y < pos.0)
                .rev()
                .find(|&(_, c)| c == '#') {
                return Some((y + 1, pos.1));
            }
        }
        (1, 0) => {
            // Get all cells below and find first obstacle
            if let Some((y, _)) = column
                .filter(|&(y, _)| y > pos.0)
                .find(|&(_, c)| c == '#') {
                return Some((y - 1, pos.1));
            }
        }
        (0, -1) => {
            // Get all cells left and find first obstacle
            if let Some((x, _)) = row
                .filter(|&(x, _)| x < pos.1)
                .rev()
                .find(|&(_, &c)| c == '#') {
                return Some((pos.0, x + 1));
            }
        }
        (0, 1) => {
            // Get all cells right and find first obstacle
            if let Some((x, _)) = row
                .filter(|&(x, _)| x > pos.1)
                .find(|&(_, &c)| c == '#') {
                return Some((pos.0, x - 1));
            }
        }
        _ => { panic!("Invalid direction {dir:?}") }
    }

    None
}

fn next_direction((y, x): (i32, i32)) -> (i32, i32) {
    (x, -y)
}

fn positions_between((y1, x1): (usize, usize), (y2, x2): (usize, usize), dir: (i32, i32)) -> Vec<(usize, usize)> {
    match dir {
        (-1, 0) => {
            (y2..=y1).map(|y| (y, x1)).collect()
        },
        (1, 0) => {
            (y1..=y2).map(|y| (y, x1)).collect()
        },
        (0, -1) => {
            (x2..=x1).map(|x| (y1, x)).collect()
        },
        (0, 1) => {
            (x1..=x2).map(|x| (y1, x)).collect()
        },
        _ => { panic!("Invalid direction {dir:?}") }
    }
}

fn final_position((y, x): (usize, usize), dir: (i32, i32), rows: usize, cols: usize) -> (usize, usize) {
    match dir {
        (-1, 0) => {
            (0, x)
        },
        (1, 0) => {
            (rows-1, x)
        },
        (0, -1) => {
            (y, 0)
        },
        (0, 1) => {
            (y, cols -1)
        },
        _ => { panic!("Invalid direction {dir:?}") }
    }
}