mod day01;

use std::path::PathBuf;
use std::time::Instant;
use crate::day01::day01;

struct Day {
    number: u32,
    input: PathBuf,
    tests: Vec<PathBuf>,
    function: Box<dyn Fn(&PathBuf) -> String>,
}

impl Day {
    pub fn new(number: u32, input: impl Into<PathBuf>, tests: Vec<impl Into<PathBuf>>, function: Box<dyn Fn(&PathBuf) -> String>) -> Box<Self> {
        let tests = tests.into_iter().map(Into::into).collect();
        Box::new(Self {number, input: input.into(), tests, function})
    }


}


fn main() {
    let days = vec![
        Day::new(1, "puzzles/day01.txt", vec!["puzzles/example01a.txt"], Box::new(day01)),
    ];
    for day in days {
        let func = &day.function;
        for (i, test) in day.tests.iter().enumerate() {
            eprintln!("Day {}: example {i}", day.number);
            let t = Instant::now();
            let solution =  func(test);
            eprintln!("\tSolution: {solution}\tTook:{:3.2?}", t.elapsed());
        }
        eprintln!("Day {}", day.number);
        let t = Instant::now();
        let solution = func(&day.input);
        eprintln!("\tSolution: {solution}\tTook:{:3.2?}", t.elapsed());
        eprintln!()
    }
}
