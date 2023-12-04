use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day02.txt");

enum CubeCount {
    Red(u32),
    Green(u32),
    Blue(u32),
}
type Round = Vec<CubeCount>;
type GameId = u32;
type Game = (GameId, Vec<Round>);
type Model = Vec<Game>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    fn parse_round(input: &str) -> nom::IResult<&str, Round> {
        separated_list1(
            tag(", "),
            alt((
                map(terminated(ncc::u32, tag(" red")), CubeCount::Red),
                map(terminated(ncc::u32, tag(" green")), CubeCount::Green),
                map(terminated(ncc::u32, tag(" blue")), CubeCount::Blue),
            )),
        )(input)
    }
    let parse_game = delimited(
        tag("Game "),
        separated_pair(ncc::u32, tag(": "), separated_list1(tag("; "), parse_round)),
        ncc::multispace1,
    );

    all_consuming(many1(parse_game))(input)
}
fn part1(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    model
        .into_iter()
        .filter_map(|(id, rounds)| {
            rounds
                .into_iter()
                .flatten()
                .all(|count| match count {
                    CubeCount::Red(n) => n <= MAX_RED,
                    CubeCount::Green(n) => n <= MAX_GREEN,
                    CubeCount::Blue(n) => n <= MAX_BLUE,
                })
                .then_some(id)
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();

    model
        .into_iter()
        .map(|(_, rounds)| {
            let (r, g, b) =
                rounds
                    .iter()
                    .flatten()
                    .fold((0, 0, 0), |(r, g, b), count| match count {
                        CubeCount::Red(n) => (r.max(*n), g, b),
                        CubeCount::Green(n) => (r, g.max(*n), b),
                        CubeCount::Blue(n) => (r, g, b.max(*n)),
                    });
            r * g * b
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day02 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day02_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 2176);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 63700);
    }
}
