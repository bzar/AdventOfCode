use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day04.txt");

type CardId = u32;
type Number = u32;
type Card = (CardId, (Vec<Number>, Vec<Number>));
type Model = Vec<Card>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_card = delimited(tag("Card"), preceded(ncc::space1, ncc::u32), ncc::char(':'));
    let parse_winning = preceded(ncc::space0, separated_list1(ncc::space1, ncc::u32));
    let parse_numbers = preceded(ncc::space0, separated_list1(ncc::space1, ncc::u32));
    let parse_line = terminated(
        tuple((
            parse_card,
            separated_pair(parse_winning, tag(" | "), parse_numbers),
        )),
        ncc::line_ending,
    );
    all_consuming(many1(parse_line))(input)
}
fn part1(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    model
        .into_iter()
        .map(|(_, (winning, numbers))| {
            numbers.into_iter().filter(|n| winning.contains(n)).count() as u32
        })
        .filter_map(|count| (count > 0).then_some(2u32.pow(count.saturating_sub(1))))
        .sum()
}
fn part2(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    let mut counts = Vec::new();
    counts.resize(model.len(), 1);
    model
        .into_iter()
        .enumerate()
        .map(move |(i, (_, (winning, numbers)))| {
            let matching = numbers.into_iter().filter(|n| winning.contains(n)).count();
            let n = counts[i];
            for j in 0..matching {
                if let Some(count) = counts.get_mut(i + j + 1) {
                    *count += n;
                }
            }
            n
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day04 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day04_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 20855);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 5489600);
    }
}
