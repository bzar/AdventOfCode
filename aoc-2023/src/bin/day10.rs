use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::terminated,
    Finish,
};
use std::collections::{HashSet, VecDeque};
const PUZZLE_INPUT: &str = include_str!("../data/day10.txt");

type Value = usize;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    East,
    South,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Start,
    FromTo([Direction; 2]),
}
struct Model(Vec<Vec<Option<Pipe>>>);

impl Direction {
    fn apply(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            North => y.checked_sub(1).map(|y| (x, y)),
            South => y.checked_add(1).map(|y| (x, y)),
            West => x.checked_sub(1).map(|x| (x, y)),
            East => x.checked_add(1).map(|x| (x, y)),
        }
    }
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}
impl Pipe {
    fn connections(&self) -> &[Direction] {
        use Direction::*;
        match self {
            Pipe::Start => &[North, West, East, South],
            Pipe::FromTo(dirs) => dirs,
        }
    }
}
impl Model {
    fn tiles(&self) -> impl Iterator<Item = (usize, usize, &Option<Pipe>)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
    }
    fn start(&self) -> Option<(usize, usize)> {
        self.tiles()
            .find(|(_, _, tile)| **tile == Some(Pipe::Start))
            .map(|(x, y, _)| (x, y))
    }
    fn at(&self, (x, y): (usize, usize)) -> Option<Pipe> {
        *self.0.get(y)?.get(x)?
    }
    fn adjacent(&self, (x, y): (usize, usize)) -> Vec<(usize, usize, Pipe)> {
        let Some(here) = self.at((x, y)) else { return Vec::new() };
        here.connections()
            .into_iter()
            .filter_map(move |to| {
                let (dx, dy) = to.apply((x, y))?;
                let at = self.at((dx, dy))?;
                at.connections()
                    .contains(&to.opposite())
                    .then_some((dx, dy, at))
            })
            .collect()
    }
}
fn parse(input: &str) -> nom::IResult<&str, Model> {
    use Direction::*;
    let parse_tile = alt((
        value(None, ncc::char('.')),
        value(Some(Pipe::Start), ncc::char('S')),
        value(Some(Pipe::FromTo([South, East])), ncc::char('F')),
        value(Some(Pipe::FromTo([South, West])), ncc::char('7')),
        value(Some(Pipe::FromTo([North, South])), ncc::char('|')),
        value(Some(Pipe::FromTo([West, East])), ncc::char('-')),
        value(Some(Pipe::FromTo([North, East])), ncc::char('L')),
        value(Some(Pipe::FromTo([North, West])), ncc::char('J')),
    ));
    map(
        all_consuming(many1(terminated(many1(parse_tile), ncc::line_ending))),
        Model,
    )(input)
}

fn part1(input: &str) -> Value {
    let (_, model) = parse(input).finish().unwrap();
    let start = model.start().unwrap();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut longest = 0;
    queue.push_back((start, 0));
    while let Some((pos, distance)) = queue.pop_front() {
        if visited.insert(pos) {
            longest = longest.max(distance);
            model
                .adjacent(pos)
                .into_iter()
                .for_each(|(x, y, _)| queue.push_back(((x, y), distance + 1)));
        }
    }
    longest
}
fn part2(input: &str) -> Value {
    let (_, model) = parse(input).finish().unwrap();
    let start = model.start().unwrap();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        if visited.insert(pos) {
            model
                .adjacent(pos)
                .into_iter()
                .for_each(|(x, y, _)| queue.push_back((x, y)));
        }
    }

    let inside: HashSet<_> = model
        .0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut inside = false;
            let mut prev_turn: Option<[Direction; 2]> = None;
            let visited = &visited;
            row.into_iter().enumerate().filter_map(move |(x, tile)| {
                use Direction::*;
                let on_pipe = visited.contains(&(x, y));
                match (inside, on_pipe, tile) {
                    (true, false, _) => Some((x, y)),
                    (_, _, Some(Pipe::FromTo([West, East]))) => None,
                    (_, true, Some(Pipe::FromTo([North, South]))) => {
                        inside = !inside;
                        None
                    }
                    (_, true, Some(Pipe::FromTo([North, East]))) => {
                        prev_turn = Some([North, East]);
                        None
                    }
                    (_, true, Some(Pipe::FromTo([South, East]))) => {
                        prev_turn = Some([South, East]);
                        None
                    }
                    (_, true, Some(Pipe::FromTo([North, West]))) => {
                        if prev_turn == Some([South, East]) {
                            inside = !inside;
                        }
                        prev_turn = None;
                        None
                    }
                    (_, true, Some(Pipe::FromTo([South, West]))) => {
                        if prev_turn == Some([North, East]) {
                            inside = !inside;
                        }
                        prev_turn = None;
                        None
                    }
                    _ => None,
                }
            })
        })
        .inspect(|x| println!("- {x:?}"))
        .collect();

    inside.len()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day10 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day10_test.txt");
    const TEST_INPUT_2: &str = include_str!("../data/day10_test_2.txt");
    const TEST_INPUT_3: &str = include_str!("../data/day10_test_3.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 6838);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 4);
    }
    #[test]
    fn test_part2_larger() {
        assert_eq!(part2(TEST_INPUT_3), 8);
    }
    #[test]
    fn test_part2_custom() {
        let map = ".............\n\
                   .....F--7....\n\
                   .....|..L--7.\n\
                   .....L7...FJ.\n\
                   ......L-S-J..\n\
                   .............\n";
        assert_eq!(part2(map), 5);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 451);
    }
}
