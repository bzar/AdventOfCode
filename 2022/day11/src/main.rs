use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as ncc,
    combinator::{map, into},
    sequence::{preceded, delimited, tuple, terminated}, multi::separated_list0, Finish,
};

#[derive(Debug)]
enum Operation {
    Add(i128),
    Mul(i128),
    Square
}
type Items = VecDeque<i128>;

#[derive(Debug)]
struct Monkey {
    items: Items,
    operation: Operation,
    test: i128,
    if_true: u32,
    if_false: u32,
    inspected: u128
}

impl From<(Vec<i128>, Operation, i128, u32, u32)> for Monkey {
    fn from((items, operation, test, if_true, if_false): (Vec<i128>, Operation, i128, u32, u32)) -> Self {
        Self { items: items.into(), operation, test, if_true, if_false, inspected: 0 }
    }
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Monkey>> {
    fn title(input: &str) -> nom::IResult<&str, u32> {
        terminated(delimited(tag("Monkey "), ncc::u32, tag(":")), ncc::line_ending)(input)
    }
    let items = delimited(tag("  Starting items: "), separated_list0(tag(", "), ncc::i128), ncc::line_ending);
    let operation = delimited(tag("  Operation: new = old "), alt((
                map(tag("* old"), |_| Operation::Square),
                map(preceded(tag("+ "), ncc::i128), |x| Operation::Add(x)),
                map(preceded(tag("* "), ncc::i128), |x| Operation::Mul(x))
                )), ncc::line_ending);
    let test = delimited(tag("  Test: divisible by "), ncc::i128, ncc::line_ending);
    let if_true = delimited(tag("    If true: throw to monkey "), ncc::u32, ncc::line_ending);
    let if_false = delimited(tag("    If false: throw to monkey "), ncc::u32, ncc::line_ending);
    let monkey = into(preceded(title, tuple((items, operation, test, if_true, if_false))));
    separated_list0(ncc::line_ending, monkey)(input)
}

fn monkey_business(mut monkeys: Vec<Monkey>, iterations: usize, worry_reduction: impl Fn(i128) -> i128) -> u128 {
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            let mut items = std::mem::take(&mut monkeys[i].items);
            for item in items.drain(0..) {
                let monkey = &mut monkeys[i];
                monkey.inspected += 1;
                let worry = worry_reduction(match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item * item
                });

                let next = (worry % monkey.test == 0)
                    .then_some(monkey.if_true)
                    .unwrap_or(monkey.if_false);

                monkeys[next as usize].items.push_back(worry);
            }
            monkeys[i].items = items;
        }
    }

    monkeys.into_iter().map(|m| m.inspected).sorted().rev().take(2).product()
}

fn part1(input: &str) -> u128 {
    let (_, monkeys) = parse(input).finish().expect("Parse error");
    monkey_business(monkeys, 20, |x| x / 3)
}

fn part2(input: &str) -> u128 {
    let (_, monkeys) = parse(input).finish().expect("Parse error");
    let div: i128 = monkeys.iter().map(|m| m.test).product();
    monkey_business(monkeys , 10_000, move |x| x % div)
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../test_input.txt")), 10605);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../test_input.txt")), 2713310158);
    }
}
