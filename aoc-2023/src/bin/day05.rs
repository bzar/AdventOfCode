use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    Finish,
};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../data/day05.txt");

type Id = u32;
type Seeds = Vec<Id>;
type Map<'a> = (&'a str, &'a str, Vec<(Id, Id, Id)>);
type Model<'a> = (Seeds, Vec<Map<'a>>);

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_seeds = preceded(tag("seeds:"), many1(preceded(ncc::space1, ncc::u32)));
    let parse_map = tuple((
        terminated(ncc::alphanumeric1, tag("-to-")),
        terminated(ncc::alphanumeric1, tag(" map:")),
        many1(preceded(
            ncc::line_ending,
            tuple((
                ncc::u32,
                preceded(ncc::space1, ncc::u32),
                preceded(ncc::space1, ncc::u32),
            )),
        )),
    ));
    all_consuming(tuple((
        parse_seeds,
        delimited(
            ncc::multispace1,
            separated_list1(ncc::multispace1, parse_map),
            ncc::multispace1,
        ),
    )))(input)
}

fn map_value(map: &Vec<(Id, Id, Id)>, value: Id) -> Id {
    map.iter()
        .find(|(_, src, range)| value >= *src && value - src < *range)
        .map(|(dst, src, _)| value - src + dst)
        .unwrap_or(value)
}
fn part1(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    let (seeds, maps) = model;
    let maps: HashMap<_, _> = maps
        .into_iter()
        .map(|(from, to, values)| (from, (to, values)))
        .collect();

    seeds
        .into_iter()
        .map(|seed| {
            let mut value = seed;
            let mut id = "seed";
            while let Some((next_id, map)) = maps.get(id) {
                value = map_value(map, value);
                id = next_id;
            }
            value
        })
        .min()
        .unwrap()
}
fn part2(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    let (seeds, maps) = model;
    let maps: HashMap<_, _> = maps
        .into_iter()
        .map(|(from, to, values)| (from, (to, values)))
        .collect();

    seeds
        .into_iter()
        .tuples()
        .flat_map(|(start, end)| start..(start + end))
        .map(|seed| {
            let mut value = seed;
            let mut id = "seed";
            while let Some((next_id, map)) = maps.get(id) {
                value = map_value(map, value);
                id = next_id;
            }
            value
        })
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day05 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day05_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 3374647);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 6082852);
    }
}
