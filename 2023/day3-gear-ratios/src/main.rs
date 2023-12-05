use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x:usize, y:usize) -> Self {
        Self { x, y,}
    }

    fn row(&self) -> usize {
        self.y
    }

    fn col(&self) -> usize {
        self.x
    }
}

#[derive(Debug)]
enum EngineInfo {
    // value, coord (start), length
    PartNumber(u64, Coordinate, usize),
    Symbol(Coordinate),
    Gear(Coordinate, u64),
}

impl EngineInfo {
    fn is_symbol(&self) -> bool {
        match self {
            EngineInfo::Symbol(_) | EngineInfo::Gear(_, _) => true,
            _ => false,
        }
    }

    fn is_gear(&self) -> bool {
        match self {
            EngineInfo::Gear(_, _) => true,
            _ => false,
        }
    }

    fn coord(&self) -> Coordinate {
        match self {
            EngineInfo::PartNumber(_, c, _) => *c,
            EngineInfo::Symbol(c) => *c,
            EngineInfo::Gear(c, _) => *c,
        }
    }

    fn value(&self) -> u64 {
        match self {
            EngineInfo::PartNumber(v, _, _) => *v,
            EngineInfo::Gear(_, ratio) => *ratio,
            _ => todo!()
        }
    }

    // set_ratio??

    fn in_range(&self, col: usize) -> bool {
        match self {
            EngineInfo::PartNumber(_, c, len) => (c.col()..(c.col()+len)).contains(&col),
            EngineInfo::Symbol(c) => c.col() == col,
            EngineInfo::Gear(c, _) => c.col() == col,
        }
    }
}

struct EngineSchematic {
    input: BufReader<File>,
    grid: Vec<Vec<EngineInfo>>
}

impl EngineSchematic {
    fn new(input: &str) -> Self {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        Self {
            input: reader,
            grid: Vec::new(),
        }
    }

    fn parse(&mut self) {
        // iterate over all bytes in the stream
        // save locations of numbers and symbols (sparse matrix?)
        let mut string = String::new();
        let mut row: usize = 0; // y
        let mut col: usize = 0; // x
        
        let mut num_coor = Coordinate::default();
        let mut num_str = String::with_capacity(10);

        while self.input.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            let mut info_list = Vec::new();
            for b in string.as_bytes() {
                if b.is_ascii_digit() {
                    // record the start of the number
                    if num_str.len() == 0 {
                        num_coor.x = col;
                        num_coor.y = row;
                    }
                    // number ends when we see '.' or symbol
                    num_str.push(*b as char);
                } else {
                    // check if we were building a number before
                    if num_str.len() > 0 {
                        info_list.push(EngineInfo::PartNumber(
                            num_str.parse().expect("num to string conversion"),
                            num_coor, num_str.len()
                        ));
                        num_str.clear();
                    }
                    if *b != b'.' && *b != b'\n'{
                        let sym = if *b == b'*' {
                            EngineInfo::Gear(Coordinate::new(col, row), 0)
                        } else {
                            EngineInfo::Symbol(Coordinate::new(col, row))
                        };
                        info_list.push(sym);
                    }
                }
                col += 1;
            }
            self.grid.push(info_list);
            // println!("found value: {}\n", val);
            row += 1;
            col = 0;
            string.clear();
        }
    }

    fn gather_part_numbers(&self) -> Vec<u64> {
        // let mut part_nums = Vec::new();
        // let mut parts = Vec::with_capacity(self.grid.len());
        let mut parts: HashMap<Coordinate, u64> = HashMap::new();
        for row in &self.grid {
            for info in row {
                if info.is_symbol() {
                    let coord = info.coord();
                    self.search_adjacents(coord, &mut parts);
                    // pick parts from grid and push onto list
                    // for part in &parts {
                    //     part_nums.push(
                    //         *part
                    //     );
                    // }
                    // parts.clear();
                }
            }
        }
        parts
            .iter()
            .map(|(_, part)| *part)
            .collect()
    }

    fn search_adjacents(&self, coord: Coordinate, locs: &mut HashMap<Coordinate, u64>) {
        // search around radius of the coordinate
        const RADIUS: [(i32, i32); 8] = [ // should proably be coords
            // (row, col)
            // upper row: ul, up, ur
            (-1, -1),
            (-1, 0),
            (-1, 1),
            // left and right
            (0, -1),
            (0, 1),
            // bottom row: bl, b, br
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        // println!("searching: {:?}", coord);

        for (row, col) in RADIUS {
            let sr = if row.is_negative() {
                coord.row().checked_sub(row.abs() as usize).unwrap()
            } else {
                coord.row().checked_add(row.abs() as usize).unwrap()
            };
            let sc = if col.is_negative() {
                coord.col().checked_sub(col.abs() as usize).unwrap()
            } else {
                coord.col().checked_add(col.abs() as usize).unwrap()
            };
            // println!("\tchecking: {}, {}", sr, sc);
            // check if we have a symbol here
            let r = &self.grid[sr];
            let adj = r
                .iter()
                .filter(|info| !info.is_symbol() && info.in_range(sc))
                .map(|num| {
                    // println!("found: {:?}", num);
                    (num.coord(), num.value())
                });
            for (c, x) in adj {
                // locs.push(x);
                // try to add in map
                // don't if it exist
                locs.insert(c, x);
            }
        }
        // self.grid[loc.row()]
        // .iter()
        // .filter(|info| info.coord().col() == loc.col())
        // .map(|x| x.value())
    }

    fn gather_gear_ratios(&self) -> Vec<u64> {
        let mut parts: HashMap<Coordinate, u64> = HashMap::new();
        let mut ratios: Vec<u64> = Vec::new();
        for row in &self.grid {
            for info in row {
                if info.is_gear() {
                    let coord = info.coord();
                    self.search_adjacents(coord, &mut parts);
                    if parts.len() == 2 {
                        ratios.push(parts
                            .iter()
                            .map(|(_, part)| *part)
                            .fold(1, |acc, part_num| acc * part_num)
                        );
                    }
                    // pick parts from grid and push onto list
                    // for part in &parts {
                    //     part_nums.push(
                    //         *part
                    //     );
                    // }
                    parts.clear();
                }
            }
        }
        ratios
    }

    fn print(&self) {
        for (num, row) in self.grid.iter().enumerate() {
            print!("row {}: ", num);
            for item in row {
                print!("{:?}, ", item);
            }
            println!("");
        }
    }
}

fn main() {
    let sum = part2();
    println!("sum: {}", sum);
}

const INPUT: &str = 
    // "test.txt";
    "input.txt";

type Answer = u64;

// we have reached a gondola!!
// - but its broken :(
// an engine part are missing??
// - add up all the numbers in the grid
// - only (part) numbers next to symbols count
fn part1() -> Answer {
    let mut schem = EngineSchematic::new(INPUT);
    // iterate over all bytes in the stream
    // save locations of numbers and symbols (sparse matrix?)
    schem.parse();
    // check sparse map for symbols next to numbers
    let part_nums = schem.gather_part_numbers();
    // convert/add up the numbers for the schematic
    // for part in part_nums.iter() {
    //     print!("{} ", part);
    // }
    // print!("\n");
    let result: u64 = part_nums
        .into_iter()
        .sum();
    result
}

// the gondala works!
// but we are going very slow :(
// - some of the gears are the wrong size!
// a gear is indicated by a *
// - adjacent to exactly two parts
// - gear ratio is result of multiplying numbers
// find the gear ratios, and add them all up
fn part2() -> Answer {
    let mut schem = EngineSchematic::new(INPUT);
    // iterate over all bytes in the stream
    // save locations of numbers and symbols (sparse matrix?)
    schem.parse();
    // check sparse map for symbols next to numbers
    let part_nums = schem.gather_gear_ratios();
    // convert/add up the numbers for the schematic
    let result: u64 = part_nums
        .into_iter()
        .sum();
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_part1() {
        let mut schem = EngineSchematic::new("test.txt");
        // iterate over all bytes in the stream
        // save locations of numbers and symbols (sparse matrix?)
        schem.parse();

        schem.print();
        // check sparse map for symbols next to numbers
        let part_nums = schem.gather_gear_ratios();
        // convert/add up the numbers for the schematic
        for part in part_nums.iter() {
            print!("{} ", part);
        }
        print!("\n");
        let result: u64 = part_nums
            .into_iter()
            .sum();
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part2() {
        let mut schem = EngineSchematic::new("test.txt");
        // iterate over all bytes in the stream
        // save locations of numbers and symbols (sparse matrix?)
        schem.parse();

        schem.print();
        // check sparse map for symbols next to numbers
        let part_nums = schem.gather_part_numbers();
        // convert/add up the numbers for the schematic
        for part in part_nums.iter() {
            print!("{} ", part);
        }
        print!("\n");
        let result: u64 = part_nums
            .into_iter()
            .sum();
        assert_eq!(result, 4361);
    }
}