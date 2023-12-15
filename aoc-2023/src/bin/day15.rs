use nom::{
    branch::alt,
    bytes::complete::take_till,
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish,
};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../data/day15.txt");

type Model<'a> = Vec<&'a str>;
#[derive(Debug)]
enum Instruction<'a> {
    Dash(&'a str),
    Set(&'a str, u32),
}

fn parse(input: &str) -> nom::IResult<&str, Model> {
    all_consuming(separated_list1(ncc::char(','), take_till(|c| c == ',')))(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    all_consuming(terminated(
        alt((
            map(
                terminated(take_till(|c| c == '-'), ncc::char('-')),
                Instruction::Dash,
            ),
            map(
                separated_pair(take_till(|c| c == '='), ncc::char('='), ncc::u32),
                |(label, value)| Instruction::Set(label, value),
            ),
        )),
        ncc::multispace0,
    ))(input)
}

fn hash(s: &str) -> u32 {
    s.bytes()
        .filter(|c| *c != '\n' as u8)
        .fold(0_u32, |acc, b| ((acc + b as u32) * 17) % 256)
}
fn part1(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    model.into_iter().map(hash).sum()
}
fn part2(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    let boxes: Vec<(HashMap<&str, usize>, Vec<u32>)> =
        (0..256).map(|_| (HashMap::new(), Vec::new())).collect();
    model
        .into_iter()
        .fold(boxes, |mut boxes, token| {
            let (_, instruction) = parse_instruction(token)
                .finish()
                .expect("Invalid instruction");
            match instruction {
                Instruction::Dash(label) => {
                    let (labels, lenses) = boxes.get_mut(hash(label) as usize).unwrap();
                    if let Some(index) = labels.remove(label) {
                        lenses.remove(index);
                        labels
                            .values_mut()
                            .filter(|x| **x > index)
                            .for_each(|x| *x -= 1);
                    }
                }
                Instruction::Set(label, value) => {
                    let (labels, lenses) = boxes.get_mut(hash(label) as usize).unwrap();
                    if let Some(index) = labels.get(label) {
                        *lenses.get_mut(*index).unwrap() = value;
                    } else {
                        let index = lenses.len();
                        lenses.push(value);
                        labels.insert(label, index);
                    }
                }
            }
            boxes
        })
        .into_iter()
        .enumerate()
        .flat_map(|(box_number, (_, lenses))| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_number, value)| {
                    (box_number as u32 + 1) * (lens_number as u32 + 1) * value
                })
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day15 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day15_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 507291);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 296921);
    }
}
