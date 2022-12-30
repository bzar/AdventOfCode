use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as ncc,
    combinator::{map, value},
    sequence::preceded,
};

#[derive(Copy, Clone)]
enum Op {
    Noop,
    AddX(i32),
}

#[derive(Default, Clone)]
struct State {
    x: i32,
    pc: i32,
}
impl State {
    fn new(x: i32, pc: i32) -> Self {
        Self { x, pc }
    }
    fn inc_pc(&self, dpc: i32) -> State {
        Self {
            x: self.x,
            pc: self.pc + dpc,
        }
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Op> + 'a {
    fn op(input: &str) -> nom::IResult<&str, Op> {
        alt((value(Op::Noop, tag("noop")), map(preceded(tag("addx "), ncc::i32), |val| Op::AddX(val))))(input)
    }
    fn newline(input: &str) -> nom::IResult<&str, &str> {
        ncc::line_ending(input)
    }
    (0..).scan(input, |input, _| {
        let (rest, o) = op(input).ok()?;
        *input = newline(rest).ok()?.0;
        Some(o)
    })
}

fn execute<'a>(program: impl Iterator<Item=Op> + 'a) -> impl Iterator<Item = State> + 'a {
    program
        .scan(State::new(1, 0), |state, op| {
            match op {
                Op::Noop => (std::mem::replace(state, state.inc_pc(1)), 1),
                Op::AddX(dx) => (
                    std::mem::replace(state, State::new(state.x + dx, state.pc + 2)),
                    2,
                ),
            }
            .into()
        })
        .flat_map(move |(s, n)| (1..=n).map(move |dpc| s.inc_pc(dpc)))
}

fn part1(input: &str) -> i32 {
    execute(parse(input))
        .skip(19)
        .step_by(40)
        .map(|s| s.pc * s.x)
        .sum()
}

fn part2(input: &str) -> String {
    const LINE_WIDTH: i32 = 40;
    let pixel = |s: State| {
        (s.x.abs_diff((s.pc - 1) % LINE_WIDTH) <= 1)
            .then_some('#')
            .unwrap_or('.')
    };
    execute(parse(input))
        .chunks(LINE_WIDTH as usize)
        .into_iter()
        .map(|chunk| chunk.map(pixel).collect::<String>())
        .join("\n")
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2:\n{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../test_input.txt")), 13140);
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(include_str!("../test_input.txt")),
            include_str!("../part2_test_output.txt").trim()
        )
    }
}
