use std::fs::File;
use std::io::{BufReader, BufRead};

struct Card {
    id: u32,
    winning: Vec<u64>,
    chosen: Vec<u64>,
}

use std::str::FromStr;
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(":");
        let card: u32 = line
            .next()
            .expect("no card")
            .split(" ")
            .last()
            .expect("no last")
            .parse()
            .unwrap();
        line = line.next().unwrap().split("|");
        let winning: Vec<u64> = line
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let chosen: Vec<u64> = line
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| {
                // println!("found: {}", n);
                n.parse().unwrap()})
            .collect();
        Ok(Card { id: card, winning, chosen })
    }
}

impl Card {
    fn num_winning(&self) -> u64 {
        self.chosen
            .iter()
            .map(|c| self.winning
                .iter()
                .any(|w| *w == *c)
            )
            .filter(|n| *n == true)
            .count() as u64
    }

    fn points(&self) -> u64 {
        Self::calculate_points(self.num_winning() - 1)
    }

    fn calculate_points(winning: u64) -> u64 {
        1 << (winning - 1)
    }
}

type PuzzleRepr = Vec<Card>;

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
            let card = Card::from_str(&string).unwrap();
            self.repr.push(card);
            string.clear();
        }
    }

    // now the gondola moves!
    // but wait, we are on the wrong island??
    // We can borrow the boat if we cound the cards
    // each card has a list of
    // - winning numbers
    // - chosen numbers
    // points calculated by doubling for each matching
    fn part1_solution(&mut self) -> Vec<u64> {
        // println!("part 1 .... ");
        let mut string = String::new();
        let mut card_points = Vec::new();
        while self.input.read_line(&mut string).unwrap() > 0 {
            // print!("{}", string);
            let card = Card::from_str(&string).unwrap();
            let num_winning: u64 = card.num_winning();
            if num_winning > 0 {
                card_points.push(Card::calculate_points(num_winning));
            }
            string.clear();
        }
        card_points
    }

    // wait, so the rules were on the back the whole time??
    // no such thing as points either ...
    // winning means more cards!!!
    // - you win n copies of cards below you ..
    // - scratch cards copies are scored like before ... 
    // - do this for every origin and copy card!
    fn part2_solution(&mut self) -> Vec<u64> {
        self.parse();
        // keep a history of which cards I have seen
        let mut history: Vec<u64> = Vec::with_capacity(self.repr.len());
        unsafe {
            history.set_len(self.repr.len())
        };
        for card in &self.repr {
            let num_winning = card.num_winning();
            // we must increment the number of cards ahead that we have seen
            // println!("checking: {}", card.id);
            let id = (card.id - 1) as usize;
            // we increment ourselves first so we know how many cards we have
            let num_cards = history[id as usize] + 1;
            history[id as usize] = num_cards;
            if num_winning > 0 {
                // then we increment n cards ahead ...
                // this is okay to do since cards that do
                // not win do not contribute to the score
                // println!("\twinning!: {}, cards: {}", num_winning, num_cards);
                for x in id+1..id+1 + num_winning as usize {
                    history[x as usize] += 1 * num_cards;
                }
            }
        }
        // println!("counting cards ...");
        // for (i, x) in history.iter().enumerate() {
        //     println!("card {} count: {}", i+1, *x);
        // }
        // then for each card calculate the score, and sum them up
        // scratch that, we only want to know the number of cards ...
        history.iter()
            .enumerate()
            // .filter(|(id, _)| self.repr[*id].num_winning() > 0)
            .map(|(id, count)| {
                // println!("{} of card {}, points: {}", count, id+1, self.repr[id].points());
                // self.repr[id].points() * count
                *count
            })
            .collect()
    }

    fn print(&self) {
        for x in self.repr.iter() {
            println!("{}", x.id);
        }
    }
}

fn main() {
    let sum = part2();
    println!("sum: {}", sum);
}

type Answer = u64;

const INPUT: &str = "input.txt";

fn part1() -> Answer {
    let mut puzzle = Puzzle::new(INPUT);
    // puzzle.parse();
    let items = puzzle.part1_solution();
    // for x in items.iter() {
    //     print!("{} ", x);
    // }
    // items.iter()
    //     .for_each(|x| print!("{}", x))
    // print!("\n");
    let result: u64 = items
        .into_iter()
        .sum();
    result
}

fn part2() -> Answer {
    let mut puzzle = Puzzle::new(INPUT);
    // puzzle.parse();
    let items = puzzle.part2_solution();
    // for x in items.iter() {
    //     print!("{} ", x);
    // }
    // print!("\n");
    let result: u64 = items
        .into_iter()
        .sum();
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        const RESULT: u64 = 13;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let items = puzzle.part1_solution();
        for x in items.iter() {
            print!("{} ", x);
        }
        print!("\n");
        let result: u64 = items
            .into_iter()
            .sum();
        assert_eq!(result, RESULT);
    }

    #[test]
    fn test_part2() {
        const RESULT: u64 = 30;
        let mut puzzle = Puzzle::new(TEST);
        // puzzle.parse();
        // puzzle.print();
        let items = puzzle.part2_solution();
        for x in items.iter() {
            print!("{} ", x);
        }
        print!("\n");
        let result: u64 = items
            .into_iter()
            .sum();
        assert_eq!(result, RESULT);
    }
}