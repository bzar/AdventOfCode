use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day06.txt");

type Time = u64;
type Distance = u64;
type Model = (Vec<Time>, Vec<Distance>);

fn parse(input: &str) -> nom::IResult<&str, Model> {
    all_consuming(tuple((
        preceded(
            tag("Time:"),
            delimited(
                ncc::space1,
                separated_list1(ncc::space1, ncc::u64),
                ncc::line_ending,
            ),
        ),
        preceded(
            tag("Distance:"),
            delimited(
                ncc::space1,
                separated_list1(ncc::space1, ncc::u64),
                ncc::line_ending,
            ),
        ),
    )))(input)
}

fn race_distance(time: Time, hold: Time) -> Distance {
    hold * time.saturating_sub(hold)
}
fn ways_to_beat_record(time: Time, record: Distance) -> u64 {
    (1..=time)
        .map(|hold| race_distance(time, hold))
        .filter(|result| result > &record)
        .count() as u64
}
fn part1(input: &str) -> u64 {
    let (_, model) = parse(input).finish().unwrap();
    let (times, distances) = model;
    let races = times.into_iter().zip(distances.into_iter());
    races
        .map(|(time, record)| ways_to_beat_record(time, record))
        .product()
}
fn part2(input: &str) -> u64 {
    let (_, model) = parse(input).finish().unwrap();
    let (times, distances) = model;
    let time: String = times.into_iter().map(|t| t.to_string()).collect();
    let distance: String = distances.into_iter().map(|d| d.to_string()).collect();
    ways_to_beat_record(time.parse().unwrap(), distance.parse().unwrap())
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day06 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day06_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 345015);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 42588603);
    }
}
