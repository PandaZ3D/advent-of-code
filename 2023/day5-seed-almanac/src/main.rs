use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Range;

use clap::Parser;
use rangemap::RangeMap;

type Answer = u64;
type PuzzleRepr = Almanac;

type Category = u64;
type CategoryMap = RangeMap<Category, Range<Category>>;

struct Almanac {
    maps: Vec<CategoryMap>,
}

impl Almanac {
    fn new() -> Almanac {
        Self { 
            maps: Vec::new(),
        }
    }

    fn convert_category(src: Category, map: &CategoryMap) -> Category {
        // attempt to see if the category has a conversion range
        if let Some((src_range, dst_range)) = map.get_key_value(&src) {
            src - src_range.start + dst_range.start
        } else {
            src
        }
    }

    fn convert_range(range: Range<Category>, map: &CategoryMap, output: &mut Vec<Range<Category>>) {
        // attempt to see if the category has a conversion range
        // we now have different cases here ...
        // 1. ranges line up perfectly
        // 2. range spills over n ranges
        
        // println!("convert: {:?}", range);
        if let Some((src_range, dst_range)) = map.get_key_value(&range.start) {
            let (start, end) = if range.end > src_range.end {
                // create new range
                // convert that the same way
                // println!("range spills over!! {:?} ({:?} -> {:?}))", range, src_range, dst_range);
                let start = src_range.start.abs_diff(range.start) + dst_range.start;
                let mid = src_range.end;            
                let end = range.end;
                Almanac::convert_range(mid..end, map, output);
                (start, dst_range.end)
            } else {
                // println!("no spillage: {:?} ({:?} -> {:?}))", range, src_range, dst_range);
                let start = src_range.start.abs_diff(range.start) + dst_range.start;            
                let end = dst_range.end - src_range.end.abs_diff(range.end);
                (start, end)
            };
            output.push(start..end);
            // println!("res: {:?} ({:?})", start..end, dst_range);
        } else {
            // println!("value not in range: {}", range.start);
            // check if the end of the range exists somewhere
            if let Some((src_range, dst_range)) = map.get_key_value(&(range.end-1)) {
                // println!("end is in range: {:?}", src_range);
                let start = range.start;
                let mid = src_range.start;
                let end = src_range.start.abs_diff(range.end) + dst_range.start; // convert this
                Almanac::convert_range(start..mid, map, output);
                output.push(dst_range.start..end);
            } else {
                // panic!("value not in range: {:?}", range);
                // println!("res: {:?}", range);
                output.push(range);    
            }
        }
    }
}

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
            repr: PuzzleRepr::new(),
        }
    }

    fn parse(&mut self) {
        let mut string = String::new();
        while self.input.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            string.clear();
        }
    }

    // sweet, we found the water source! (Island Island)
    // the gardener says there is not enough sand to filter it :(
    // realizes they kinda forgot its been off for a while!
    // looks like we gotta check out what's up with the sand
    //
    // lol now we have to solve their food production problem
    // goal: the gardener wants us to find out lowest location 
    // for seeds to be planted
    // - we do this by translating the seed in the almanac to a location
    // the alamanac contains maps from one type to another
    // - map lists translation from src category to dst category
    // - describes ranges: dst, src, len
    // - numbers not in the map are 1-1
    fn part1_solution(&mut self) -> Answer {
        let mut string = String::new();
        
        // read the first line, that's our seeds
        let _  = self.input.read_line(&mut string);
        let mut items: Vec<Category> = string
            .split(":")
            .last()
            .expect("no last")
            .trim()
            .split(" ")
            .map(|s| s.parse().expect("failed to convert seed"))
            .collect();
        // skip newline
        let _ = self.input.read_line(&mut string);
        string.clear();

        // a way to track the map that we are making
        let mut map = CategoryMap::new();
        let mut range: Vec<Category>  = Vec::with_capacity(3);
        // now we parse the maps and convert along the way
        while self.input.read_line(&mut string).unwrap() > 0 {
            if string.contains("map") {
                // println!("found map!!!");
            }
            // end map condition
            else if string == "\n" {
                // println!("end map!!!");
                // convert the category we are on
                for src in items.iter_mut() {
                    *src = Almanac::convert_category(*src, &map);
                }
                map = CategoryMap::new();
            } else {
                // parse a range
                range.extend(string
                    .trim()
                    .split(" ")
                    .map(|n| n.parse::<Category>().expect("failed to parse category"))
                );
                let (dst, src, len) = (range[0], range[1], range[2]);
                map.insert(src..src+len, dst..dst+len);
                range.clear();
            }
            // print!("{}", string);
            string.clear();
        }

        // print out final items (location)
        // items.iter().for_each(|c| print!("{} ", *c));
        // println!("");
        *items.iter()
            .min()
            .unwrap()
    }

    // turns out the seed line means a range of seeds!!
    // - the values come in pairs (start, length)
    // - now we consider all numbers in that range
    // - still find the nearest location
    fn part2_solution(&mut self) -> Answer {
        let mut string = String::new();
        
        // read the first line, that's our seeds
        let _  = self.input.read_line(&mut string);
        let mut ping_to_pong: bool = false;
        let mut ping: Vec<Range<Category>> = Vec::new();
        let mut pong: Vec<Range<Category>> = string
            .split(":")
            .last()
            .expect("no last")
            .trim()
            .split(" ")
            .map(|s| s.parse().expect("failed to convert seed"))
            .collect::<Vec<u64>>()
            .as_slice()
            .chunks(2)
            // .inspect(|x| println!("{} {}", x[0], x[1]))
            .map(|x| {
                let start = x[0];
                let end = x[1];
                start..start+end
            })
            .collect();

        // skip newline
        let _ = self.input.read_line(&mut string);
        string.clear();

        // a way to track the map that we are making
        let mut map = CategoryMap::new();
        let mut range: Vec<Category>  = Vec::with_capacity(3);
        // now we parse the maps and convert along the way
        while self.input.read_line(&mut string).unwrap() > 0 {
            if string.contains("map") {
                // println!("found map!!!");
                // print!("{}", string);
            }
            // end map condition
            else if string == "\n" {
                // println!("end map!!!");
                // convert the category we are on
                if ping_to_pong {
                    for src in ping.iter_mut() {
                        Almanac::convert_range(src.clone(), &map, &mut pong);
                    }
                    ping.clear();
                    ping_to_pong = false;
                } else {
                    for src in pong.iter_mut() {
                        Almanac::convert_range(src.clone(), &map, &mut ping);
                    }
                    pong.clear();
                    ping_to_pong = true;
                }
                // break;
                map = CategoryMap::new();
            } else {
                // parse a range
                range.extend(string
                    .trim()
                    .split(" ")
                    .map(|n| n.parse::<Category>().expect("failed to parse category"))
                );
                let (dst, src, len) = (range[0], range[1], range[2]);
                map.insert(src..src+len, dst..dst+len);
                range.clear();
            }
            // print!("{}", string);
            string.clear();
        }

        // print out final items (location)
        if !ping_to_pong {
            // pong.iter().for_each(|c| print!("{:?} ", *c));
            pong.iter()
                .map(|r| r.clone().min().unwrap())
                .min()
                .unwrap()
        } else {
            // ping.iter().for_each(|c| print!("{:?} ", *c));
            ping.iter()
                .map(|r| r.clone().min().unwrap())
                .min()
                .unwrap()
        }
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
        const RESULT: u64 = 35;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let result = puzzle.part1_solution();
        assert_eq!(result, RESULT);
    }

    #[test]
    fn test_part2() {
        const RESULT: u64 = 46;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let result = puzzle.part2_solution();
        assert_eq!(result, RESULT);
    }
}