use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    combinator::all_consuming,
    multi::fold_many1,
    sequence::{separated_pair, terminated, tuple},
    Finish,
};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../data/day08.txt");

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;
type Model<'a> = (&'a str, Map<'a>);

fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Model<'a>> {
    let parse_map_line = tuple((
        terminated(ncc::alphanumeric1, tag(" = (")),
        terminated(ncc::alphanumeric1, tag(", ")),
        terminated(ncc::alphanumeric1, tag(")\n")),
    ));
    all_consuming(separated_pair(
        ncc::not_line_ending,
        tag("\n\n"),
        fold_many1(
            parse_map_line,
            HashMap::new,
            |mut acc: HashMap<_, _>, (key, left, right)| {
                acc.insert(key, (left, right));
                acc
            },
        ),
    ))(input)
}
fn follow<'a>(
    start: &'a str,
    instructions: &'a str,
    map: &'a Map<'a>,
) -> impl Iterator<Item = &'a str> + 'a {
    instructions.chars().cycle().scan(start, |pos, turn| {
        let (left, right) = map.get(pos)?;
        let result: &str = pos;
        *pos = match turn {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };
        Some(result)
    })
}
fn part1(input: &str) -> usize {
    let (_, (instructions, map)) = parse(input).finish().unwrap();
    follow("AAA", instructions, &map)
        .take_while(|pos| *pos != "ZZZ")
        .count()
}
fn find_pattern(start: &str, instructions: &str, map: &Map) -> (usize, usize) {
    let mut ends = follow(start, instructions, &map)
        .enumerate()
        .filter(|(_, pos)| pos.ends_with('Z'));

    let (first, _) = ends.next().unwrap();
    let (second, _) = ends.next().unwrap();
    let (third, _) = ends.next().unwrap();
    assert_eq!(second - first, third - second);
    (first, second - first)
}
fn combine_patterns(
    (start1, repeat1): (usize, usize),
    (start2, repeat2): (usize, usize),
) -> (usize, usize) {
    let mut common = (0..)
        .scan((0, 0, (1..), (1..)), |(n1, n2, i1, i2), _| {
            let result = (*n1, *n2);
            if start1 + *n1 * repeat1 < start2 + *n2 * repeat2 {
                *n1 = i1.next().unwrap();
            } else {
                *n2 = i2.next().unwrap();
            }
            Some(result)
        })
        .filter(|(n1, n2)| start1 + n1 * repeat1 == start2 + n2 * repeat2);
    let first = common.next().map(|(n, _)| start1 + repeat1 * n).unwrap();
    let second = common.next().map(|(n, _)| start1 + repeat1 * n).unwrap();
    let third = common.next().map(|(n, _)| start1 + repeat1 * n).unwrap();
    assert_eq!(second - first, third - second);
    (first, second - first)
}
fn part2(input: &str) -> usize {
    let (_, (instructions, map)) = parse(input).finish().unwrap();
    let starts: Vec<_> = map.keys().filter(|key| key.ends_with('A')).collect();
    let patterns: Vec<_> = starts
        .into_iter()
        .map(|start| find_pattern(start, instructions, &map))
        .collect();

    let (first, _) = patterns.into_iter().reduce(combine_patterns).unwrap();

    first
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day08_test.txt");
    const TEST_INPUT_2: &str = include_str!("../data/day08_test_2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 19951);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 6);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 16342438708751);
    }
}
