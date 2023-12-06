use std::fs::File;
use std::io::{BufReader, BufRead};

use clap::Parser;

type Answer = u64;
type PuzzleRepr = Vec<u64>;

struct Puzzle {
    input: BufReader<File>,
    repr: PuzzleRepr,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        Self {
            input: reader,
            repr: Vec::new(),
        }
    }

    fn parse(&mut self) {
        let mut string = String::new();
        while self.input.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            string.clear();
        }
    }

    fn part1_solution(&mut self) -> Answer {
        todo!()
    }

    fn part2_solution(&mut self) -> Answer {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
struct Args {
    /// Run Part 1
    #[arg(short('f'), long)]
    run_part_1: bool,

    /// Run Part 2
    #[arg(short('s'), long)]
    run_part_2: bool,
}

fn main() {
    // clap args parsing
    let args = Args::parse();
    if args.run_part_1 {
        let ans = part1();
        println!("part 1: {}", ans);
    }
    if args.run_part_2 {
        let ans = part2();
        println!("part 2: {}", ans);
    }
}

const INPUT: &str = "input.txt";

fn part1() -> Answer {
    let mut puzzle = Puzzle::new(INPUT);
    // puzzle.parse();
    let result = puzzle.part1_solution();
    result
}

fn part2() -> Answer {
    let mut puzzle = Puzzle::new(INPUT);
    // puzzle.parse();
    let result = puzzle.part2_solution();
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        const RESULT: u64 = 0;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let result = puzzle.part1_solution();
        assert_eq!(result, RESULT);
    }

    #[test]
    fn test_part2() {
        const RESULT: u64 = 0;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let result = puzzle.part2_solution();
        assert_eq!(result, RESULT);
    }
}
