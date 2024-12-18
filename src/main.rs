mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use crate::day01::day01;
use glob::glob;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::day10::day10;
use crate::day11::day11;

#[derive(Parser)]
struct Cli {
    /// The day to run
    day: Option<usize>,
    /// Run the latest day
    #[arg(long, default_value_t = false)]
    latest: bool,
    /// Debug mode: only run examples
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Only run actual inputs, no examples
    #[arg(long, default_value_t = false)]
    puzzle_only: bool,
}

struct Day {
    number: u32,
    input: PathBuf,
    tests: Vec<PathBuf>,
    function: Box<dyn Fn(&PathBuf) -> String>,
}

impl Day {
    pub fn new(number: u32, function: Box<dyn Fn(&PathBuf) -> String>) -> Box<Self> {
        let input_filename = format!("puzzles/day{number:02}.txt");
        let mut input_matches = glob(&input_filename).unwrap();
        let input = input_matches.next().expect(&format!("Cannot find file {input_filename}")).unwrap();

        let test_filename = format!("puzzles/example{number:02}*.txt");
        let test_matches = glob(&test_filename).unwrap();
        let tests = test_matches.into_iter().map(|m| m.unwrap()).collect();
        Box::new(Self { number, input, tests, function })
    }
}


fn main() {
    let days = vec![
        Day::new(1, Box::new(day01)),
        Day::new(2, Box::new(day02)),
        Day::new(3, Box::new(day03)),
        Day::new(4, Box::new(day04)),
        Day::new(5, Box::new(day05)),
        Day::new(6, Box::new(day06)),
        Day::new(7, Box::new(day07)),
        Day::new(8, Box::new(day08)),
        Day::new(9, Box::new(day09)),
        Day::new(10, Box::new(day10)),
        Day::new(11, Box::new(day11)),
    ];

    let args = Cli::parse();

    if args.latest {
        eprintln!("Running latest day:");
        benchmark(days.last().unwrap(), &args);
        return;
    }

    if let Some(num) = args.day {
        eprintln!("Running day {num}:");
        benchmark(&days[num - 1], &args);
        return;
    }

    eprintln!("Running all days:");
    for day in days {
        benchmark(&day, &args);
    }
}

fn benchmark(day: &Day, args: &Cli) {
    let func = &day.function;
    if !args.puzzle_only {
        eprintln!("Day {}:", day.number);
        for (i, test) in day.tests.iter().enumerate() {
            let t = Instant::now();
            let solution = func(test);
            eprintln!("\tExample {}\tSolution: {solution}\tTook:{:3.2?}", i + 1, t.elapsed());
        }
    }

    if args.debug { return; }

    let t = Instant::now();
    let solution = func(&day.input);
    let prefix = if args.puzzle_only { format!("Day {}", day.number) } else { "\tPuzzle".into() };
    eprintln!("{prefix}\t\tSolution: {solution}\tTook:{:3.2?}", t.elapsed());
    if !args.puzzle_only { eprintln!() }
}
