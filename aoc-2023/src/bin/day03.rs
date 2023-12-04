use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map, map_parser, value},
    multi::many1,
    sequence::terminated,
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day03.txt");

#[derive(Clone, Copy, Debug)]
enum Symbol<'a> {
    Number(&'a str),
    Other(char),
}

impl<'a> Symbol<'a> {
    fn len(&self) -> usize {
        match self {
            Symbol::Number(s) => s.len(),
            Symbol::Other(_) => 1,
        }
    }
}
type Line<'a> = Vec<Option<Symbol<'a>>>;
type Model<'a> = Vec<Line<'a>>;
type Coord = usize;
type Coords = (Coord, Coord);

fn symbols<'a>(model: &'a Model) -> impl Iterator<Item = (Coords, Symbol<'a>)> {
    model
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().scan(0, move |x, symbol| {
                let s_x = *x;
                *x += symbol.map(|s| s.len()).unwrap_or(1);
                Some(((s_x, y), symbol))
            })
        })
        .filter_map(|(coords, &symbol)| Some((coords, symbol?)))
}
fn adjacent<'a>(
    model: &'a Model,
    (x, y): Coords,
    s_len: Coord,
) -> impl Iterator<Item = Symbol<'a>> {
    let y0 = y.saturating_sub(1);
    let x0 = x.saturating_sub(1);
    let y1 = y + 1;
    let x1 = x + s_len;

    symbols(model)
        .skip_while(move |((_, y), _)| *y < y0)
        .take_while(move |((_, y), _)| *y <= y1)
        .filter(move |((x, _), s)| *x + s.len() - 1 >= x0 && *x <= x1)
        .map(|(_, s)| s)
}

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_symbol = alt((
        value(None, ncc::char('.')),
        map(ncc::digit1, |c| Some(Symbol::Number(c))),
        map(ncc::anychar, |c| Some(Symbol::Other(c))),
    ));
    let parse_line = terminated(
        map_parser(ncc::not_line_ending, many1(parse_symbol)),
        ncc::line_ending,
    );
    all_consuming(many1(parse_line))(input)
}
fn part1(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();
    symbols(&model)
        .filter_map(|(coords, symbol)| match symbol {
            Symbol::Number(x) => Some((coords, x)),
            Symbol::Other(_) => None,
        })
        .filter_map(|(coords, number)| {
            adjacent(&model, coords, number.len())
                .any(|s| matches!(s, Symbol::Other(_)))
                .then_some(number)
        })
        .map(|n| u32::from_str_radix(n, 10).unwrap())
        .sum()
}
fn part2(input: &str) -> u32 {
    let (_, model) = parse(input).finish().unwrap();

    symbols(&model)
        .filter(|(_, symbol)| matches!(symbol, Symbol::Other('*')))
        .filter_map(|(coords, symbol)| {
            let mut adj_numbers = adjacent(&model, coords, symbol.len()).filter_map(|s| match s {
                Symbol::Number(x) => Some(x),
                Symbol::Other(_) => None,
            });
            let first = adj_numbers.next()?;
            let second = adj_numbers.next()?;
            if let Some(_) = adj_numbers.next() {
                return None;
            }
            Some(u32::from_str_radix(first, 10).unwrap() * u32::from_str_radix(second, 10).unwrap())
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day03 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day03_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 536202);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 78272573);
    }
}
