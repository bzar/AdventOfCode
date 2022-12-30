use itertools::Itertools;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl FromStr for RPS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

static WINS: [(RPS, RPS); 3] = [
    (RPS::Rock, RPS::Scissors),
    (RPS::Paper, RPS::Rock),
    (RPS::Scissors, RPS::Paper),
];
fn score(game: (RPS, RPS)) -> i32 {
    match game {
        (a, b) if a == b => Outcome::Draw as i32 + b as i32,
        (a, b) if WINS.contains(&(b, a)) => Outcome::Win as i32 + b as i32,
        (_, b) => Outcome::Lose as i32 + b as i32,
    }
}

fn strategy((opponent, outcome): (RPS, Outcome)) -> (RPS, RPS) {
    let choice = match outcome {
        Outcome::Win => WINS.iter().find(|(_, b)| *b == opponent).unwrap().0,
        Outcome::Lose => WINS.iter().find(|(a, _)| *a == opponent).unwrap().1,
        Outcome::Draw => opponent,
    };
    (opponent, choice)
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).expect("Error reading data");

    let part1: i32 = input
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .tuples()
        .map(score)
        .sum();

    let part2: i32 = input
        .split_whitespace()
        .tuples()
        .map(|(opponent, outcome)| (opponent.parse().unwrap(), outcome.parse().unwrap()))
        .map(strategy)
        .map(score)
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
