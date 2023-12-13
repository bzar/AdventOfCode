use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    Finish,
};
use rayon::prelude::*;
const PUZZLE_INPUT: &str = include_str!("../data/day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Working,
    Broken,
    Unknown,
}
type Spring = Vec<Part>;
type Groups = Vec<u32>;
type Model = Vec<(Spring, Groups)>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_spring = many1(alt((
        value(Part::Working, ncc::char('.')),
        value(Part::Broken, ncc::char('#')),
        value(Part::Unknown, ncc::char('?')),
    )));
    let parse_groups = separated_list1(ncc::char(','), ncc::u32);
    all_consuming(many1(terminated(
        separated_pair(parse_spring, ncc::space1, parse_groups),
        ncc::line_ending,
    )))(input)
}

fn valid_up_to(spring: &Spring, groups: &Groups, limit: usize) -> bool {
    if spring.len() < limit {
        return false;
    }
    let mut iter = spring.iter().take(limit).peekable();
    for group in groups {
        if iter.peek().is_none() {
            return true;
        }
        while iter.peek() == Some(&&Part::Working) {
            iter.next();
            if iter.peek().is_none() {
                return true;
            }
        }
        for _ in 0..*group {
            if iter.peek().is_none() {
                return true;
            }
            if iter.next() != Some(&Part::Broken) {
                return false;
            }
        }
        if iter.peek().is_none() {
            return true;
        }
        if iter.peek() == Some(&&Part::Broken) {
            return false;
        }
    }
    return iter.all(|part| *part == Part::Working);
}
fn match_spring(spring: &Spring, groups: &Groups) -> bool {
    let mut iter = spring.iter().peekable();
    for group in groups {
        while iter.peek() == Some(&&Part::Working) {
            iter.next();
        }
        for _ in 0..*group {
            if iter.next() != Some(&Part::Broken) {
                return false;
            }
        }
        if iter.peek() == Some(&&Part::Broken) {
            return false;
        }
    }
    return iter.all(|part| *part == Part::Working);
}
fn spring_configurations<'a>(
    spring: &'a Spring,
    groups: &'a Groups,
) -> impl Iterator<Item = Spring> + 'a {
    let total_broken: usize = groups.iter().sum::<u32>() as usize;
    let mut stack = Vec::new();
    stack.push(spring.clone());
    let mut next = move || -> Option<Spring> {
        while !stack.is_empty() {
            let mut s = stack.pop()?;
            while s.iter().filter(|part| **part == Part::Broken).count() > total_broken
                || s.iter().filter(|part| **part != Part::Working).count() < total_broken
            {
                s = stack.pop()?;
            }
            if let Some((index, _)) = s.iter().find_position(|p| **p == Part::Unknown) {
                for value in [Part::Working, Part::Broken] {
                    let mut s = s.clone();
                    s[index] = value;
                    if valid_up_to(&s, &groups, index + 1) {
                        stack.push(s);
                    }
                }
            } else {
                return Some(s);
            }
        }
        None
    };

    (0..)
        .map(move |_| next())
        .take_while(|s| s.is_some())
        .map(|s| s.unwrap())
}

fn arrangements(spring: &Spring, groups: &Groups) -> u128 {
    spring_configurations(&spring, &groups)
        .filter(|s| match_spring(s, &groups))
        .fold(0u128, |sum, _solution| sum + 1)
}

fn part1(input: &str) -> u128 {
    let (_, model) = parse(input).finish().unwrap();
    model
        .into_iter()
        .map(|(spring, groups)| arrangements(&spring, &groups))
        .sum()
}
fn unfold_spring(spring: &Spring, groups: &Groups, n: usize) -> (Spring, Groups) {
    let mut unfolded_spring = Spring::new();
    (0..(n - 1)).for_each(|_| {
        unfolded_spring.extend_from_slice(&spring);
        unfolded_spring.push(Part::Unknown);
    });
    unfolded_spring.extend_from_slice(&spring);

    (
        unfolded_spring,
        groups
            .iter()
            .copied()
            .cycle()
            .take(n * groups.len())
            .collect(),
    )
}
fn unfolded_arrangements(spring: &Spring, groups: &Groups) -> u128 {
    let (s2, g2) = unfold_spring(spring, groups, 2);
    let (s3, g3) = unfold_spring(spring, groups, 3);
    let n2 = arrangements(&s2, &g2);
    let n3 = arrangements(&s3, &g3);
    let mid = n3 / n2;

    n2 * mid.pow(3)
}
fn part2(input: &str) -> u128 {
    let (_, model) = parse(input).finish().unwrap();
    let n = model.len();
    model
        .par_iter()
        .enumerate()
        .map(|(i, x)| {
            println!("{i}/{n}");
            x
        })
        .map(|(spring, groups)| unfolded_arrangements(&spring, &groups))
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day12 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day12_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 7694);
    }
    #[test]
    fn test_part2_custom() {
        assert_eq!(part2("???.### 1,1,3\n"), 1);
        assert_eq!(part2(".??..??...?##. 1,1,3\n"), 16384);
        assert_eq!(part2("?#?#?#?#?#?#?#? 1,3,1,6\n"), 1);
        assert_eq!(part2("????.#...#... 4,1,1\n"), 16);
        assert_eq!(part2("????.######..#####. 1,6,5\n"), 2500);
        assert_eq!(part2("?###???????? 3,2,1\n"), 506250);
        assert_eq!(
            part2("?#?#?#?#?#?#?#? 1,3,1,6\n?###???????? 3,2,1\n"),
            506251
        );
        assert_eq!(part1("?..#..### 1,3\n"), 1);
        assert_eq!(part1("...?????.?? 1,2,1\n"), 6);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }

    /*
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 544723432977);
    }
    */
}
