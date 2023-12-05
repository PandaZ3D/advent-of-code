// some digits spelled out with numbers!
// same as before, find the first and last, sum

use std::fs::File;
use std::io::{BufReader, BufRead};

struct CalibrationDocument {
    file: BufReader<File>,
    values: Vec<u64>,
}

const NUM_STR_TABLE: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

impl CalibrationDocument {
    fn new(input: &str) -> CalibrationDocument {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);    
        CalibrationDocument {
            file: reader,
            values: Vec::new(),
        }
    }

    fn unscramble(&mut self) {
        let mut string = String::new();
        while self.file.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            let val = CalibrationDocument::get_value(&string);
            self.values.push(val as u64);
            // println!("found value: {}\n", val);
            string.clear();
            // break;
        }
    }

    fn get_value(line: &String) -> u8 {
        // let mut nums = line
        //     .iter()
        //     .filter(|b| b.is_ascii_digit());
        let to_u8 = |b: &u8| b - 48;
        // let first = nums
        //     .clone()
        //     .peekable()
        //     .peek()
        //     .map(|b| to_u8(*b))
        //     .unwrap();
        // let last = nums
        //     .last()
        //     .map(to_u8)
        //     .unwrap();
        
        let mut anchor = 0;
        let mut first_found = false;
        let mut first = 0;
        let mut last = None;
        for (i, b) in line.as_bytes().iter().enumerate() {
            // anchor
            // iter
            // move cursor

            // substring or digit?
            let v = if b.is_ascii_digit() {
                // if we have find an int reset anchor
                anchor = i + 1;
                Some(to_u8(b))
            } else {
                // scan all substrings to see if we have a number
                let mut n = None;
                'give: for left in anchor..i+1 {
                    for right in left..i+1 {
                        // try convert
                        // println!("{} {} i={}", left, right, i);
                        let x = line.get(left..=right).unwrap();
                        // println!("{}, {:?}", x, NUM_STR_TABLE.iter().position(|&n| n == x));
                        let num = NUM_STR_TABLE
                            .iter()
                            .position(|&n| n == x)
                            .map(|n| n as u8);
                        if let Some(_) = num {
                            n = num;
                            // reset anchor
                            // anchor = right + 1;           
                            // break 'give
                        }
                    }
                }
                n
            };
            if let Some(num) = v {
                if !first_found {
                    first = num;
                    first_found = true;
                } else {
                    last = Some(num);
                }
            }
        }
        (first * 10) + last.unwrap_or(first)
    }

    fn calibrate(&self) -> u64 {
        self.values.as_slice().iter().sum()
    }
}


const INPUT: &str = "input.txt";
fn main() {
    let mut doc = CalibrationDocument::new(INPUT);
    doc.unscramble();
    println!("sum: {}", doc.calibrate());

    // tests
    // 1fourfiveljrmbmfpsvzzhdlh
    // qseven4eight1fivejcrt
    // rvgmtnjsix9hvncrblxfour176
    let case1 = String::from("rvgmtnjsix9hvncrblxfour176");
    let case2 = String::from("qseven4eight1fivejcrt");
    let case3 = String::from("1fourfiveljrmbmfpsvzzhdlh");
    let case4 = String::from("twone");
    println!("\n{}\nval: {}", case1, CalibrationDocument::get_value(&case1));
    println!("\n{}\nval: {}", case2, CalibrationDocument::get_value(&case2));
    println!("\n{}\nval: {}", case3, CalibrationDocument::get_value(&case3));
    println!("\n{}\nval: {}", case4, CalibrationDocument::get_value(&case4));
}
