use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{map, all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
type Coord = usize;
type Position = (Coord, Coord);
enum Direction { East = 0, South = 1, West = 2, North = 3 }
struct Character {
    position: Position,
    heading: Direction
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Floor,
    Empty
}

type Map = Vec<Vec<Cell>>;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Advance(u32),
    Left,
    Right
}
type Instructions = Vec<Instruction>;
type Data = (Map, Instructions);

impl Character {
    fn advance(&mut self, map: &Map) {
        let (x, y) = self.position;
        use Cell::Empty;
        let mut next = match self.heading {
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::North => (x, y - 1),
        };
        if map.get(next.1).map(|row| row.get(next.0)).flatten().map(|c| c == &Empty).unwrap_or(true) {
            match self.heading {
                Direction::East => {
                    next.0 = map[next.1].iter().enumerate().find_map(|(i, c)| (c != &Empty).then_some(i)).unwrap();
                },
                Direction::South => {
                    next.1 = map.iter().enumerate().find_map(|(i, row)| (row.get(next.0)? != &Empty).then_some(i)).unwrap();
                },
                Direction::West => {
                    next.0 = map[next.1].iter().enumerate().rev().find_map(|(i, c)| (c != &Empty).then_some(i)).unwrap();
                },
                Direction::North => {
                    next.1 = map.iter().enumerate().rev().find_map(|(i, row)| (row.get(next.0)? != &Empty).then_some(i)).unwrap();
                }
            }
        }
        if map[next.1][next.0] == Cell::Floor {
            self.position = next;
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }
    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}
fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data> {
    use Cell::*;
    let map_line = many1(alt((
        value(Wall, ncc::char('#')),
        value(Floor, ncc::char('.')),
        value(Empty, ncc::char(' ')),
    )));
    let map_data = separated_list1(ncc::line_ending, map_line);
    let instructions = many1(alt((
                map(ncc::u32, Instruction::Advance),
                value(Instruction::Left, ncc::char('L')),
                value(Instruction::Right, ncc::char('R')),
                )));
    let data = separated_pair(map_data, ncc::multispace1, instructions);
    all_consuming(terminated(data, ncc::multispace0))(input)
}

fn part1((map, instructions): &Data) -> usize {
    let start_x = map[0].iter().enumerate().find_map(|(i, c)| (*c == Cell::Floor).then_some(i)).expect("No start position");
    let mut character = Character { position: (start_x, 0), heading: Direction::East };
    for instruction in instructions {
        match instruction {
            Instruction::Advance(x) => (0..*x).for_each(|_| character.advance(map)),
            Instruction::Left => character.turn_left(),
            Instruction::Right => character.turn_right(),
        }
    }
    let (x, y) = character.position;
    (y + 1) * 1000 + (x + 1) * 4 + character.heading as Coord
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
    fn test_part1() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 6032);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 75388);
    }
    #[test]
    fn test_part2() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 5031);
    }
    /*
    #[test]
    fn test_part2_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 711);
    }
    */
}

