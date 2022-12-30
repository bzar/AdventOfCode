use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    Finish,
};

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum Value {
    Literal(i32),
    List(Vec<Value>),
}
type Pairs = Vec<(Value, Value)>;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn cmp_lists(l: &[Value], r: &[Value]) -> Option<Ordering> {
            l.iter()
                .zip(r.iter())
                .filter_map(|(li, ri)| li.partial_cmp(ri))
                .filter(|o| o.is_ne())
                .next()
                .or(Some(l.len().cmp(&r.len())))
        }

        use Value::*;
        match (self, other) {
            (Literal(l), Literal(r)) => l.partial_cmp(r),
            (List(l), List(r)) => cmp_lists(l, r),
            (Literal(l), List(r)) => cmp_lists(&[Literal(*l)], r),
            (List(l), Literal(r)) => cmp_lists(l, &[Literal(*r)]),
        }
    }
}

fn parse(input: &str) -> nom::IResult<&str, Pairs> {
    fn value(i: &str) -> nom::IResult<&str, Value> {
        let values = separated_list0(ncc::char(','), value);
        let value_list = delimited(ncc::char('['), values, ncc::char(']'));
        alt((map(ncc::i32, Value::Literal), map(value_list, Value::List)))(i)
    }
    let packet = |i| terminated(value, ncc::line_ending)(i);
    separated_list0(ncc::line_ending, tuple((packet, packet)))(input)
}

fn part1(pairs: &Pairs) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| Some((i, l.partial_cmp(r)?)))
        .filter_map(|(i, o)| (o.is_le()).then_some(i + 1))
        .sum()
}

fn part2(pairs: Pairs) -> usize {
    let filters = [
        Value::List(vec![Value::List(vec![Value::Literal(2)])]),
        Value::List(vec![Value::List(vec![Value::Literal(6)])]),
    ];
    let mut values: Vec<_> = pairs
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .chain(filters.iter().cloned())
        .collect();

    values.sort();

    values
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| filters.contains(&v).then_some(i + 1))
        .product()
}

fn main() {
    let (_, pairs) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(pairs));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, pairs) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&pairs), 13);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, pairs) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&pairs), 5720);
    }
    #[test]
    fn test_part2() {
        let (_, pairs) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(pairs), 140);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, pairs) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(pairs), 23504);
    }
}
