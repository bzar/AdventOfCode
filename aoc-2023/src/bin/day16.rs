use aoc_2023::{Coords, Direction, RectMap};
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::terminated,
    Finish,
};
use rayon::prelude::*;
use std::collections::HashSet;

const PUZZLE_INPUT: &str = include_str!("../data/day16.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    LeftMirror,
    RightMirror,
    HorizontalSplitter,
    VerticalSplitter,
}
type Model = RectMap<Tile>;
type Beam = (Coords, Direction);

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_tile = alt((
        value(Tile::Empty, ncc::char('.')),
        value(Tile::LeftMirror, ncc::char('\\')),
        value(Tile::RightMirror, ncc::char('/')),
        value(Tile::HorizontalSplitter, ncc::char('-')),
        value(Tile::VerticalSplitter, ncc::char('|')),
    ));
    all_consuming(map(
        many1(terminated(many1(parse_tile), ncc::line_ending)),
        RectMap::new,
    ))(input)
}

fn energize(model: &Model, beam: &Beam) -> usize {
    let mut stack: Vec<Beam> = vec![*beam];
    let mut previous = HashSet::new();
    let mut energized = HashSet::new();
    while let Some((pos, dir)) = stack.pop() {
        if previous.contains(&(pos, dir)) {
            continue;
        }
        if let Some(tile) = model.get(pos) {
            energized.insert(pos);
            previous.insert((pos, dir));
            use Direction::*;
            use Tile::*;
            let dirs: &[Direction] = match (tile, dir) {
                (LeftMirror, East) => &[South],
                (LeftMirror, North) => &[West],
                (LeftMirror, West) => &[North],
                (LeftMirror, South) => &[East],
                (RightMirror, East) => &[North],
                (RightMirror, North) => &[East],
                (RightMirror, West) => &[South],
                (RightMirror, South) => &[West],
                (HorizontalSplitter, North | South) => &[East, West],
                (VerticalSplitter, East | West) => &[North, South],
                (_, East) => &[East],
                (_, North) => &[North],
                (_, West) => &[West],
                (_, South) => &[South],
            };

            dirs.into_iter()
                .filter_map(|d| Some((d.apply(pos)?, *d)))
                .for_each(|beam| stack.push(beam));
        }
    }
    energized.len()
}
fn part1(input: &str) -> usize {
    let (_, model) = parse(input).finish().unwrap();
    energize(&model, &((0, 0), Direction::East))
}
fn part2(input: &str) -> usize {
    let (_, model) = parse(input).finish().unwrap();
    let east = (0..model.height()).map(|y| ((0, y), Direction::East));
    let north = (0..model.width()).map(|x| ((x, model.height() - 1), Direction::North));
    let west = (0..model.height()).map(|y| ((model.width() - 1, y), Direction::West));
    let south = (0..model.width()).map(|x| ((x, 0), Direction::South));
    east.chain(north)
        .chain(west)
        .chain(south)
        .par_bridge()
        .map(|beam| energize(&model, &beam))
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day16 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day16_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 7111);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 7831);
    }
}
