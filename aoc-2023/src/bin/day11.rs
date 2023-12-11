use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::many1,
    sequence::terminated,
    Finish,
};
use std::collections::HashSet;
const PUZZLE_INPUT: &str = include_str!("../data/day11.txt");

type Model = Vec<Vec<bool>>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    all_consuming(many1(terminated(
        many1(alt((
            value(true, ncc::char('#')),
            value(false, ncc::char('.')),
        ))),
        ncc::line_ending,
    )))(input)
}

fn part1(input: &str) -> usize {
    let (_, model) = parse(input).finish().unwrap();
    let columns_to_expand: HashSet<_> = (0..model[0].len())
        .filter(|x| !model.iter().any(|row| *row.get(*x).unwrap_or(&false)))
        .collect();
    let expanded: Model = model
        .into_iter()
        .flat_map(|row| {
            let new_row: Vec<_> = row
                .iter()
                .enumerate()
                .flat_map(|(x, &v)| {
                    if columns_to_expand.contains(&x) {
                        [v].into_iter().cycle().take(2)
                    } else {
                        [v].into_iter().cycle().take(1)
                    }
                })
                .collect();
            if !row.iter().any(|x| *x) {
                [new_row].into_iter().cycle().take(2)
            } else {
                [new_row].into_iter().cycle().take(1)
            }
        })
        .collect();

    let galaxies: Vec<_> = expanded
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, v)| v.then_some((x, y)))
        })
        .collect();

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}
fn part2(input: &str) -> u64 {
    let (_, model) = parse(input).finish().unwrap();
    let rows_to_expand: HashSet<_> = model
        .iter()
        .enumerate()
        .filter_map(|(y, row)| (!row.iter().any(|x| *x)).then_some(y))
        .collect();
    let columns_to_expand: HashSet<_> = (0..model[0].len())
        .filter(|x| !model.iter().any(|row| *row.get(*x).unwrap_or(&false)))
        .collect();

    let galaxies: Vec<_> = model
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, v)| v.then_some((x, y)))
        })
        .collect();

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| -> u64 {
            let dx: u64 = (x1.min(x2)..x1.max(x2))
                .map(|x| {
                    if columns_to_expand.contains(&x) {
                        1_000_000
                    } else {
                        1
                    }
                })
                .sum();
            let dy: u64 = (y1.min(y2)..y1.max(y2))
                .map(|y| {
                    if rows_to_expand.contains(&y) {
                        1_000_000
                    } else {
                        1
                    }
                })
                .sum();
            dx + dy
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day11 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day11_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 374);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 9522407);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 82000210);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 544723432977);
    }
}
