mod day01;
mod day02;
mod day03;

use crate::day01::day01;
use glob::glob;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use crate::day02::day02;
use crate::day03::day03;

#[derive(Parser)]
struct Cli {
    /// The day to run
    day: Option<usize>,
    /// Run the latest day
    #[arg(long, default_value_t = false)]
    latest: bool,
    /// Debug mode: only run examples
    #[arg(long, default_value_t = false)]
    debug: bool
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
    eprintln!("Day {}:", day.number);
    for (i, test) in day.tests.iter().enumerate() {
        let t = Instant::now();
        let solution = func(test);
        eprintln!("\t Example {}\tSolution: {solution}\tTook:{:3.2?}", i + 1, t.elapsed());
    }
    
    if args.debug { return; }
    
    let t = Instant::now();
    let solution = func(&day.input);
    eprintln!("\t Puzzle\t\tSolution: {solution}\tTook:{:3.2?}", t.elapsed());
    eprintln!()
}
