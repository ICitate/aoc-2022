use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<char>>
}

impl Crane {
    fn apply(&mut self, m: &Move) {
        for _ in 0..m.amount {
            let item = self.stack[m.from].pop().expect("should have an item");
            self.stack[m.to].push(item);
        }
    }

    fn apply_9001(&mut self, m: &Move) {
        for i in (1..=m.amount).rev() {
            let length = self.stack[m.from].len();
            let item = self.stack[m.from].remove(length - i);
            self.stack[m.to].push(item);
        }
    }

    fn print_tops(&self) {
        for stack in &self.stack {
            print!("{}", stack.last().expect("must have some element"));
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCraneErr;

impl FromStr for Crane {
    type Err = ParseCraneErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();

        for line in s.lines().rev() {

            for (crane_idx, idx) in (0..line.len()).step_by(4).enumerate() {
                if stack.len() <= crane_idx {
                    stack.push(Vec::new());
                }

                if line[idx..].starts_with('[') {
                    let item = line.chars().nth(idx + 1).expect("should exist");
                    stack[crane_idx].push(item);
                }
            }
        }

        Ok(Crane { stack })
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Move {
            amount: iter.next().expect("must exist"),
            from: iter.next().expect("must exist") - 1,
            to: iter.next().expect("must exist") - 1,
        } 
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveErr;

impl FromStr for Move {
    type Err = ParseMoveErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {     
       return Ok(s
               .split_whitespace()
               .flat_map(|x| x.parse::<usize>())
               .collect::<Move>());
    }
}

fn main() {
    let input = read_to_string("input.prod")
        .expect("Should have been able to read file");

    let (crane_str, moves_str) = input.split_at(input.find("\n\n").unwrap() + 2);

    part_one(crane_str, moves_str);
    part_two(crane_str, moves_str);
}

fn part_one(crane_str: &str, moves_str: &str) {
    let mut crane: Crane = crane_str.parse().expect("Crane parsing err");

    let moves: Vec<Move> = moves_str
        .lines()
        .map(|l| l.parse().expect("Move parsing error"))
        .collect();

    for m in moves {
        crane.apply(&m);
    }

    println!("Crane after moves were applied: (Part one)");
    crane.print_tops();
}

fn part_two(crane_str: &str, moves_str: &str) {
    let mut crane: Crane = crane_str.parse().expect("Crane parsing err");

    let moves: Vec<Move> = moves_str
        .lines()
        .map(|l| l.parse().expect("Move parsing error"))
        .collect();

    for m in moves {
        crane.apply_9001(&m);
    }

    println!("Crane after moves were applied: (Part two)");
    crane.print_tops();
}
