// a big bag o cubes
// record random draws from bag
// find out which games are possible from the record
// - 12 red, 13 green, 14 blue

// cubes are ints, max in game is fixed
// draws may br inacurate based on constraints

use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RedCube(u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GreenCube(u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BlueCube(u64);

impl RedCube {
    fn get(&self) -> u64 {
        self.0
    }
}

impl GreenCube {
    fn get(&self) -> u64 {
        self.0
    }
}

impl BlueCube {
    fn get(&self) -> u64 {
        self.0
    }
}
struct Game {
    id: usize,
    draws: Vec<(RedCube, GreenCube, BlueCube)>,
}

impl Game {
    fn new(record: &String) -> Self {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut info = record.trim().split(":");
        let id = info
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        // println!("Game: {}", id);
        let draws: Vec<_> = info
            .next()
            .unwrap()
            .split(";")
            .map(|game| {
                let cubes = game.split(",");
                let mut red = RedCube(0);
                let mut green = GreenCube(0);
                let mut blue = BlueCube(0);
                for x in cubes {
                    // println!("{}", x);
                    let mut draw = x
                    .trim()
                    .split(" ");
                    let num = draw.next().unwrap().parse().unwrap();
                    // println!("n={:?}", num);
                    match draw.next().unwrap().trim() {
                        "green" => green = GreenCube(num),
                        "red" => red = RedCube(num),
                        "blue" => blue = BlueCube(num),
                        _ => {},
                    }
                }
                // what if multiple of each color???
                // println!("---");
                // return tuple
                (red, green, blue)
            })
            .collect();
            Self {
                id,
                draws,
            }
        }

    fn print(&self) {
        print!("{}:", self.id);
        for (r, g, b) in &self.draws {
            print!("r={},g={},b={}; ", r.get(), g.get(), b.get());
        }
        println!("");
    }

    fn possible(&self, r: RedCube, g: GreenCube, b: BlueCube) -> bool {
        // is this game possible???
        self.draws
            .iter()
            .all(|(rd, gd, bd)| {
                rd.get() <= r.get() && gd.get() <= g.get() && bd.get() <= b.get()
            })
    }

    fn min_cubes(&self) -> (RedCube, GreenCube, BlueCube) {
        // get the max cubes seen in drawn for each color
        let mut r = RedCube(0);
        let mut g = GreenCube(0);
        let mut b = BlueCube(0);
        for (rd, gd, bd) in &self.draws {
            if *rd > r {
                r = RedCube(rd.get())
            }
            if *gd > g {
                g = GreenCube(gd.get())
            }
            if *bd > b {
                b = BlueCube(bd.get())
            }
        }
        (r,g,b)
    }
}

struct CubeRecords {
    file: BufReader<File>,
    games: Vec<Game>,
}

impl CubeRecords {
    fn new(input: &str) -> Self {
        // load data
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        Self {
            file: reader,
            games: Vec::new()
        }
    }

    fn load_game_records(&mut self) {
        // parse the games
        let mut string = String::new();
        while self.file.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            self.games.push(Game::new(&string));
            // println!("found value: {}\n", val);
            string.clear();
        }
    }

    fn print_games(&self) {
        for game in &self.games {
            game.print()
        }
    }

    fn possible_games_sum(&self, max_red: RedCube, max_green: GreenCube, max_blue: BlueCube) -> u64 {
        // tokenize and parse diffrent games 
        // generate a list of possible games
        // let possible = Vec::new();
        self.games
            .iter()
            .filter(|g| g.possible(max_red, max_green, max_blue))
            .map(|g| g.id as u64)
            .sum()
        // sum them up
        // 0
    } 

    fn possible_games_power(&self, max_red: RedCube, max_green: GreenCube, max_blue: BlueCube) -> u64 {
        // tokenize and parse diffrent games 
        // generate a list of possible games
        // let possible = Vec::new();
        self.games
            .iter()
            // .filter(|g| g.possible(max_red, max_green, max_blue))
            .map(|g| {
                g.print();
                let (r,g,b) = g.min_cubes();
                r.get() * g.get() * b.get()
            })
            .sum()
        // sum them up
        // 0
    } 
}

const INPUT: &str = 
    // "test.txt";
    "input.txt";

// no snow produced due to water ... 
// what is the min amount of info needed in possible games?
// - for each cube find the min cubes in bag
// - based on max observed in the draws

fn main() {
    let mut records = CubeRecords::new(INPUT);
    records.load_game_records();
    // records.print_games();
    // let sum = records.possible_games_sum(
    //     RedCube(12),
    //     GreenCube(13),
    //     BlueCube(14)
    // );
    let sum = records.possible_games_power(
        RedCube(12),
        GreenCube(13),
        BlueCube(14)
    );
    println!("sum: {}", sum);
}
