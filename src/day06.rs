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
    let direction = (-1, 0);

    for (y, line) in reader.lines().enumerate() {
        let line: Vec<_> = line.unwrap().chars().collect();
        if let Some(x) = line.iter().position(|&x| x == '^') {
            position = (y, x);
        }
        grid.push(line);
    }
    
    let grid = Grid::new(grid);

    let mut positions = HashSet::new();
    let mut obstacle_options = HashSet::new();

    simulate(&grid, direction, position, &mut positions, &mut obstacle_options, None);
    // There cannot be an obstacle at the initial position of the guard:
    obstacle_options.remove(&position);

    format!("Part one: {}\t Part two: {}", positions.len(), obstacle_options.len())
}

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Self { rows: grid.len(), cols: grid[0].len(), grid }
    }
}

/// Returns true on loop, false on exit
fn simulate(grid: &Grid, initial_direction: (i32, i32), initial_position: (usize, usize),
            positions: &mut HashSet<(usize, usize)>,
            obstacle_options: &mut HashSet<(usize, usize)>,
            new_obstacle: Option<(usize, usize)>,
) -> bool {
    let mut finished = false;
    let mut direction = initial_direction;
    let mut position = initial_position;
    let mut tried_obstacle_options = HashSet::new();
    let mut positions_with_dir = HashSet::new();

    while !finished {
        let next_pos = if let Some(next_pos) = get_next_obstacle(&grid.grid, direction, position, new_obstacle) {
            next_pos
        } else {
            finished = true;
            final_position(position, direction, grid.rows, grid.cols)
        };

        for intermediate_pos in positions_between(position, next_pos, direction) {
            positions.insert(intermediate_pos);
            if !positions_with_dir.insert((intermediate_pos, direction)) {
                // If insert returns false, this position and direction have already been visited
                // i.e. we have a loop
                return true;
            }

            // If we didn't already add an obstacle and if this position has not been tried yet
            if new_obstacle.is_none() && tried_obstacle_options.insert((intermediate_pos, direction)) {
                // Try if inserting here will give a loop
                let new_obstacle = get_offset(intermediate_pos, direction, 1);
                let simulation = simulate(grid, initial_direction, initial_position, &mut HashSet::new(), obstacle_options, Some(new_obstacle));
                if simulation {
                    // If the new situation contains a loop, this is a valid obstacle location
                    obstacle_options.insert(new_obstacle);
                }
            }
        }
        position = next_pos;
        direction = next_direction(direction);
    }
    // No loop, return false
    false
}

fn get_next_obstacle(grid: &[Vec<char>], dir: (i32, i32), pos: (usize, usize), extra_obstacle: Option<(usize, usize)>) -> Option<(usize, usize)> {
    let column = grid.iter().map(|r| r[pos.1]).enumerate().map(|(y, c)| (y, pos.1, c));
    let row = grid[pos.0].iter().enumerate().map(|(x, &c)| (pos.0, x, c));
    match dir {
        (-1, 0) => {
            // Get all cells above and find first obstacle
            if let Some((y, x, _)) = column
                .filter(|&(y, _, _)| y < pos.0)
                .rev()
                .find(|&(y, x, c)| is_obstacle((y, x), c, extra_obstacle)) {
                return Some((y + 1, x));
            }
        }
        (1, 0) => {
            // Get all cells below and find first obstacle
            if let Some((y, x, _)) = column
                .filter(|&(y, _, _)| y > pos.0)
                .find(|&(y, x, c)| is_obstacle((y, x), c, extra_obstacle)) {
                return Some((y - 1, x));
            }
        }
        (0, -1) => {
            // Get all cells left and find first obstacle
            if let Some((y, x, _)) = row
                .filter(|&(_, x, _)| x < pos.1)
                .rev()
                .find(|&(y, x, c)| is_obstacle((y, x), c, extra_obstacle)) {
                return Some((y, x + 1));
            }
        }
        (0, 1) => {
            // Get all cells right and find first obstacle
            if let Some((y, x, _)) = row
                .filter(|&(_, x, _)| x > pos.1)
                .find(|&(y, x, c)| is_obstacle((y, x), c, extra_obstacle)) {
                return Some((y, x - 1));
            }
        }
        _ => { panic!("Invalid direction {dir:?}") }
    }

    None
}

fn is_obstacle(pos: (usize, usize), c: char, extra_obstacle: Option<(usize, usize)>) -> bool {
    if c == '#' { return true; }
    if let Some(obs) = extra_obstacle {
        if obs == pos { return true; }
    }
    false
}

fn next_direction((y, x): (i32, i32)) -> (i32, i32) {
    (x, -y)
}

fn positions_between((y1, x1): (usize, usize), (y2, x2): (usize, usize), dir: (i32, i32)) -> Vec<(usize, usize)> {
    match dir {
        (-1, 0) => {
            (y2..=y1).map(|y| (y, x1)).collect()
        }
        (1, 0) => {
            (y1..=y2).map(|y| (y, x1)).collect()
        }
        (0, -1) => {
            (x2..=x1).map(|x| (y1, x)).collect()
        }
        (0, 1) => {
            (x1..=x2).map(|x| (y1, x)).collect()
        }
        _ => { panic!("Invalid direction {dir:?}") }
    }
}

fn final_position((y, x): (usize, usize), dir: (i32, i32), rows: usize, cols: usize) -> (usize, usize) {
    match dir {
        (-1, 0) => {
            (0, x)
        }
        (1, 0) => {
            (rows - 1, x)
        }
        (0, -1) => {
            (y, 0)
        }
        (0, 1) => {
            (y, cols - 1)
        }
        _ => { panic!("Invalid direction {dir:?}") }
    }
}

fn get_offset((y, x): (usize, usize), dir: (i32, i32), offset: i32) -> (usize, usize) {
    match dir {
        (-1, 0) => {
            ((y as i32 - offset) as usize, x)
        }
        (1, 0) => {
            ((y as i32 + offset) as usize, x)
        }
        (0, -1) => {
            (y, (x as i32 - offset) as usize)
        }
        (0, 1) => {
            (y, (x as i32 + offset) as usize)
        }
        _ => { panic!("Invalid direction {dir:?}") }
    }
}
