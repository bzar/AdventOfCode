use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    Finish,
};
use rayon::prelude::*;
const PUZZLE_INPUT: &str = include_str!("../data/day13.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}
type Map = Vec<Vec<Tile>>;
type Model = Vec<Map>;

#[derive(Debug, PartialEq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}
fn parse(input: &str) -> nom::IResult<&str, Model> {
    fn parse_map(input: &str) -> nom::IResult<&str, Map> {
        let parse_tile = alt((
            value(Tile::Ash, ncc::char('.')),
            value(Tile::Rock, ncc::char('#')),
        ));
        many1(terminated(many1(parse_tile), ncc::line_ending))(input)
    }
    all_consuming(separated_list1(ncc::line_ending, parse_map))(input)
}

fn rows<'a>(
    map: &'a Map,
) -> impl Iterator<Item = impl Iterator<Item = &'a Tile> + Clone + core::fmt::Debug>
       + 'a
       + DoubleEndedIterator
       + ExactSizeIterator
       + core::fmt::Debug {
    map.iter().map(|row| row.iter())
}
fn columns<'a>(
    map: &'a Map,
) -> impl Iterator<Item = impl Iterator<Item = &'a Tile> + Clone + core::fmt::Debug>
       + 'a
       + DoubleEndedIterator
       + ExactSizeIterator
       + core::fmt::Debug {
    // Assume each row has equal length
    (0..map[0].len()).map(|x| map.iter().map(move |row| &row[x]))
}
fn find_reflections(map: &Map) -> impl Iterator<Item = Reflection> + core::fmt::Debug + '_ {
    let vertical = rows(map)
        .tuple_windows()
        .enumerate()
        .filter_map(|(i, (a, b))| a.zip(b).all(|(a, b)| a == b).then_some(i))
        .filter(|i| {
            rows(map)
                .take(i + 1)
                .rev()
                .zip(rows(map).skip(i + 1))
                .all(|(a, b)| a.zip(b).all(|(a, b)| a == b))
        })
        .map(|i| Reflection::Vertical(i + 1));
    let horizontal = columns(map)
        .tuple_windows()
        .enumerate()
        .filter_map(|(i, (a, b))| a.zip(b).all(|(a, b)| a == b).then_some(i))
        .filter(|i| {
            columns(map)
                .take(i + 1)
                .rev()
                .zip(columns(map).skip(i + 1))
                .all(|(a, b)| a.zip(b).all(|(a, b)| a == b))
        })
        .map(|i| Reflection::Horizontal(i + 1));

    vertical.chain(horizontal)
}
fn find_reflection_with_smudge(map: &Map) -> Option<Reflection> {
    fn flip(tile: &Tile) -> Tile {
        match tile {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        }
    }
    let coords = (0..map[0].len()).cartesian_product(0..map.len());
    let smudged_reflections = coords.flat_map(|(x, y)| {
        let mut m = map.clone();
        m[y][x] = flip(&m[y][x]);
        find_reflections(&m).collect::<Vec<_>>().into_iter()
    });

    let unsmudged_reflection = find_reflections(map).next()?;
    smudged_reflections
        .filter(|r| *r != unsmudged_reflection)
        .next()
}
fn part1(input: &str) -> usize {
    let (_, model) = parse(input).finish().unwrap();
    model
        .par_iter()
        .map(|map| find_reflections(&map).exactly_one().unwrap())
        .map(|r| match r {
            Reflection::Horizontal(x) => x,
            Reflection::Vertical(x) => x * 100,
        })
        .sum()
}
fn part2(input: &str) -> usize {
    let (_, model) = parse(input).finish().unwrap();
    model
        .par_iter()
        .map(|map| find_reflection_with_smudge(&map).unwrap())
        .map(|r| match r {
            Reflection::Horizontal(x) => x,
            Reflection::Vertical(x) => x * 100,
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day13 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day13_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 405);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 33780);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 400);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 23479);
    }
}
