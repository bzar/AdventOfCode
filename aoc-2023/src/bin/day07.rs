use itertools::Itertools;
use nom::{
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::{many1, many_m_n},
    sequence::{separated_pair, terminated},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day07.txt");

const CARD_VALUES: &str = "234566789TJQKA";
const CARD_VALUES_WITH_JOKERS: &str = "J234566789TQKA";
type Bid = u64;
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Card(usize);
type Hand = Vec<Card>;
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Rank {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
type Model = Vec<(Hand, Bid)>;

fn parse<'a>(input: &'a str, card_values: &str) -> nom::IResult<&'a str, Model> {
    let parse_hand = many_m_n(
        5,
        5,
        map(ncc::one_of(card_values), |c| {
            Card(card_values.find(c).unwrap())
        }),
    );
    all_consuming(many1(terminated(
        separated_pair(parse_hand, ncc::char(' '), ncc::u64),
        ncc::line_ending,
    )))(input)
}

fn rank_hand(hand: &Hand) -> Rank {
    let mut counts = [0; CARD_VALUES.len()];
    hand.iter().for_each(|Card(i)| counts[*i] += 1);
    let max_count = counts.iter().max().unwrap();
    match max_count {
        5 => Rank::FiveOfAKind,
        4 => Rank::FourOfAKind,
        3 if counts.iter().contains(&2usize) => Rank::FullHouse,
        3 => Rank::ThreeOfAKind,
        2 if counts.iter().filter(|c| **c == 2).count() == 2 => Rank::TwoPair,
        2 => Rank::OnePair,
        _ => Rank::HighCard,
    }
}
fn rank_hand_with_jokers(hand: &Hand) -> Rank {
    let mut counts = [0; CARD_VALUES_WITH_JOKERS.len()];
    hand.iter().for_each(|Card(i)| counts[*i] += 1);
    let jokers = counts[0];
    let counts = &counts[1..];
    let max_count = counts.iter().max().unwrap() + jokers;
    let pairs = counts.iter().filter(|c| **c == 2).count();
    match max_count {
        5 => Rank::FiveOfAKind,
        4 => Rank::FourOfAKind,
        3 if pairs - jokers == 1 => Rank::FullHouse,
        3 => Rank::ThreeOfAKind,
        2 if pairs == 2 => Rank::TwoPair,
        2 => Rank::OnePair,
        _ => Rank::HighCard,
    }
}
fn part1(input: &str) -> u64 {
    let (_, model) = parse(input, CARD_VALUES).finish().unwrap();
    model
        .into_iter()
        .map(|(hand, bid)| ((rank_hand(&hand), hand), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum()
}
fn part2(input: &str) -> u64 {
    let (_, model) = parse(input, CARD_VALUES_WITH_JOKERS).finish().unwrap();
    model
        .into_iter()
        .map(|(hand, bid)| ((rank_hand_with_jokers(&hand), hand), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day07_test.txt");
    const TEST_INPUT_2: &str = include_str!("../data/day07_test_2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 250474325);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 5905);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 248909434);
    }
}
