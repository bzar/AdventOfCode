use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    Finish,
};
const PUZZLE_INPUT: &str = include_str!("../data/day09.txt");

type Value = i64;
type History = Vec<Value>;
type Model = Vec<History>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    all_consuming(many1(terminated(separated_list1(ncc::space1, ncc::i64), ncc::line_ending)))(input)
}

fn part1(input: &str) -> Value {
    let (_, model) = parse(input).finish().unwrap();
    model.into_iter().flat_map(|history| {
        (0..).scan(history, |h: &mut History, _| {
            if h.iter().all(|x| *x == 0) {
                None
            } else {
                let result: i64 = *h.last().unwrap();
                *h = h.iter().tuple_windows().map(|(a, b)| b - a).collect();
                Some(result)
            }
        })
    })
    .sum()
}
fn part2(input: &str) -> Value {
    let (_, model) = parse(input).finish().unwrap();
    model.into_iter().map(|history| {
        (0..).scan(history, |h: &mut History, _| {
            if h.iter().all(|x| *x == 0) {
                None
            } else {
                let result: i64 = *h.first().unwrap();
                *h = h.iter().tuple_windows().map(|(a, b)| b - a).collect();
                Some(result)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .reduce(|a, b| b - a)
        .unwrap()
    })
    .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day09 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day09_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 1584748274);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 1026);
    }
}
