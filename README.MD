# Advent of Code 2024

My solutions to [Advent of Code 2024](https://adventofcode.com/2024). Written in Rust.

Put puzzles and examples in the `puzzles` folder.  

## Usage
To run all days:
```shell
cargo run --release --
```
To run latest day:
```shell
cargo run --release -- --latest
```
To run day x:
```shell
cargo run --release -- x
```
Additionally, you can provide `--debug` to only run the examples
of a day and skip running the full input.  
Or you can provide `--puzzle-only` to only run the full input and no examples.

# Benchmarks:
```
Day 1           252.11µs
Day 2           442.99µs
Day 3           161.42µs
Day 4           2.24ms
Day 5           4.00ms
Day 6           2.84s
Day 7           66.22ms
Day 8           265.85µs
Day 9           2.03s
Day 10          1.84ms
```
