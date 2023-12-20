use aoc_2023::{Coord, Coords, Direction};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day18.txt");

type Amount = u32;
type Color<'a> = &'a str;
type Model<'a> = Vec<(Direction, Amount, Color<'a>)>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_direction = alt((
        value(Direction::East, ncc::char('R')),
        value(Direction::North, ncc::char('U')),
        value(Direction::West, ncc::char('L')),
        value(Direction::South, ncc::char('D')),
    ));
    let parse_line = tuple((
        terminated(parse_direction, ncc::space1),
        terminated(ncc::u32, ncc::space1),
        delimited(tag("(#"), take_until(")"), tag(")")),
    ));
    all_consuming(many1(terminated(parse_line, ncc::line_ending)))(input)
}

fn polygon_area(vertices: Vec<Coords>) -> Coord {
    // Basic polygon area
    let inner = vertices
        .iter()
        .tuple_windows()
        .map(|([x0, y0], [x1, y1])| x0 * y1 - x1 * y0)
        .sum::<Coord>()
        .abs()
        / 2;
    // Add padding to outer edges
    let outer = vertices
        .iter()
        .tuple_windows()
        .map(|([x0, y0], [x1, y1])| (x0.abs_diff(*x1) as Coord + y0.abs_diff(*y1) as Coord + 1))
        .sum::<Coord>()
        / 2
        - (vertices.len() as Coord - 1) / 2
        + 1; // sum of padding corners
    inner + outer
}
fn part1(input: &str) -> Coord {
    let (_, model) = parse(input).finish().unwrap();
    let start = [0, 0];
    let vertices = model.into_iter().scan(start, |pos, (dir, n, _)| {
        *pos = dir.apply_n(*pos, n as Coord).expect("invalid coordinates");
        Some(*pos)
    });
    let trench: Vec<_> = [[0, 0]].into_iter().chain(vertices).collect();
    polygon_area(trench)
}

fn decode_hexcode(s: &str) -> (Direction, u32) {
    assert!(s.len() == 6);
    let d = match &s[5..] {
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        "3" => Direction::North,
        _ => unreachable!(),
    };
    let n = u32::from_str_radix(&s[..5], 16).expect("invalid hex code");
    (d, n)
}
fn part2(input: &str) -> Coord {
    let (_, model) = parse(input).finish().unwrap();
    let start = [0, 0];
    let vertices = model.into_iter().scan(start, |pos, (_, _, color)| {
        let (dir, n) = decode_hexcode(color);
        *pos = dir.apply_n(*pos, n as Coord).expect("invalid coordinates");
        Some(*pos)
    });
    let trench: Vec<_> = [[0, 0]].into_iter().chain(vertices).collect();
    polygon_area(trench)
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day18 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day18_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 62);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 34329);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 42617947302920);
    }
}
