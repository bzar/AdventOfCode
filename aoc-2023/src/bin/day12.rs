use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    Finish,
};
use rayon::prelude::*;
use std::collections::HashMap;
const PUZZLE_INPUT: &str = include_str!("../data/day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Working,
    Broken,
    Unknown,
}
type Spring = Vec<Part>;
type Group = u32;
type Groups = Vec<Group>;
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

fn arrangements(spring: &Spring, groups: &Groups) -> u128 {
    fn hash(spring: &[Part], groups: &[Group], broken_count: u32) -> (String, usize, u32) {
        let spring_chars: String = spring
            .iter()
            .map(|p| match p {
                Part::Working => '.',
                Part::Broken => '#',
                Part::Unknown => '?',
            })
            .collect();
        (spring_chars, groups.len(), broken_count)
    }
    fn arrangements_recursive(
        spring: &[Part],
        groups: &[Group],
        broken_count: u32,
        memo: &mut HashMap<(String, usize, u32), u128>,
    ) -> u128 {
        if let Some(result) = memo.get(&hash(spring, groups, broken_count)) {
            return *result;
        }
        let result = match (spring.first(), groups.first()) {
            (None, Some(n)) if *n != broken_count || groups.len() != 1 => 0,
            (None, _) => 1,
            (Some(Part::Broken), None) => 0,
            (Some(Part::Broken), Some(n)) if broken_count >= *n => 0,
            (Some(Part::Working), None) if broken_count == 0 => {
                arrangements_recursive(&spring[1..], groups, 0, memo)
            }
            (Some(Part::Working), None) => 0,
            (Some(Part::Working), Some(n)) if broken_count != 0 && *n == broken_count => {
                arrangements_recursive(&spring[1..], &groups[1..], 0, memo)
            }
            (Some(Part::Working), Some(_)) if broken_count != 0 => 0,
            (Some(Part::Working), Some(_)) => arrangements_recursive(&spring[1..], groups, 0, memo),
            (Some(Part::Broken), Some(_)) => {
                arrangements_recursive(&spring[1..], groups, broken_count + 1, memo)
            }
            (Some(Part::Unknown), _) => {
                let mut a: Vec<_> = spring.iter().copied().collect();
                a[0] = Part::Working;
                let mut b: Vec<_> = spring.iter().copied().collect();
                b[0] = Part::Broken;
                let na = arrangements_recursive(&a, groups, broken_count, memo);
                let nb = arrangements_recursive(&b, groups, broken_count, memo);
                na + nb
            }
        };
        memo.insert(hash(spring, groups, broken_count), result);
        result
    }
    let mut memo = HashMap::new();
    arrangements_recursive(spring, groups, 0, &mut memo)
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
fn part2(input: &str) -> u128 {
    let (_, model) = parse(input).finish().unwrap();
    model
        .par_iter()
        .map(|(spring, groups)| unfold_spring(&spring, &groups, 5))
        .map(|(spring, groups)| arrangements(&spring, &groups))
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

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 5071883216318);
    }
}
