use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

struct Snafu(Vec<i64>);
type Data = Vec<Snafu>;

fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data> {
    let line = map(
        many1(alt((
            value(-2, ncc::char('=')),
            value(-1, ncc::char('-')),
            value(0, ncc::char('0')),
            value(1, ncc::char('1')),
            value(2, ncc::char('2')),
        ))),
        Snafu,
    );
    let data = separated_list1(ncc::line_ending, line);
    all_consuming(delimited(ncc::multispace0, data, ncc::multispace0))(input)
}

impl From<&Snafu> for i64 {
    fn from(value: &Snafu) -> Self {
        value
            .0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * 5i64.pow(i as u32))
            .sum::<i64>()
    }
}

impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        let digits = 2 + (value as f64).log(5.0).ceil() as u32;
        let snafu = (0..digits)
            .map(|i| {
                let m = 5i64.pow(i);
                value % (5 * m) / m
            })
            .scan(0, |carry, value| {
                let digit = *carry + value;
                if digit > 2 {
                    *carry = 1;
                    Some(digit - 5)
                } else {
                    *carry = 0;
                    Some(digit)
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        Self(snafu.skip_while(|v| *v == 0).collect())
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| match x {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            })
            .collect()
    }
}
fn part1(data: &Data) -> String {
    let sum = data.iter().map(i64::from).sum::<i64>();
    Snafu::from(sum).to_string()
}

fn part2(data: &Data) -> usize {
    todo!()
}

fn main() {
    let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_encode() {
        let values = [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (decimal, snafu) in values {
            assert_eq!(&Snafu::from(decimal).to_string(), snafu);
        }
    }
    #[test]
    fn test_part1() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(&part1(&data), "2=-1=0");
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), "2-1=10=1=1==2-1=-221");
    }
    /*
    #[test]
    fn test_part2() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 20);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 986);
    }
    */
}
