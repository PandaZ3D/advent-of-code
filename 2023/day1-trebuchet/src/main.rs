// input 
// calibration document
// calibration val on each line
// - find value (two digit number)
// - made by first and last
// find sum of all calibration values

use std::fs::File;
use std::io::{BufReader, BufRead};

const INPUT: &str = "input.txt";

struct CalibrationDocument {
    file: BufReader<File>,
    values: Vec<u64>,
}

impl CalibrationDocument {
    fn new(input: &str) -> CalibrationDocument {
        let file = File::open(input).unwrap();
        let mut reader = BufReader::new(file);    
        CalibrationDocument {
            file: reader,
            values: Vec::new(),
        }
    }

    fn unscramble(&mut self) {
        let mut string = String::new();
        while self.file.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            let val = CalibrationDocument::get_value(&string.as_bytes());
            self.values.push(val as u64);
            // println!("found value: {}\n", val);
            string.clear();
        }
    }

    fn get_value(line: &[u8]) -> u8 {
        let mut nums = line
            .iter()
            .filter(|b| b.is_ascii_digit());
        let to_u8 = |b: &u8| b - 48;
        let first = nums
            .clone()
            .peekable()
            .peek()
            .map(|b| to_u8(*b))
            .unwrap();
        let last = nums
            .last()
            .map(to_u8)
            .unwrap();
        (first * 10) + last
    }

    fn calibrate(&self) -> u64 {
        self.values.as_slice().iter().sum()
    }
}

fn main() {
    let mut doc = CalibrationDocument::new(INPUT);
    doc.unscramble();
    println!("sum: {}", doc.calibrate())
}
