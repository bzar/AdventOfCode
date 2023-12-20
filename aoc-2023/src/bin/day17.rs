use aoc_2023::{astar, Coord, Coords, Direction, RectMap};
use nom::{
    bytes::complete::take,
    character::complete as ncc,
    combinator::{all_consuming, map, map_parser},
    multi::many1,
    sequence::terminated,
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day17.txt");

type Cost = u32;
type Model = RectMap<Cost>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_tile = map_parser(take(1usize), ncc::u32);
    all_consuming(map(
        many1(terminated(many1(parse_tile), ncc::line_ending)),
        RectMap::new,
    ))(input)
}

fn part1(input: &str) -> Cost {
    let (_, model) = parse(input).finish().unwrap();
    let start = [0, 0];
    let goal = [
        model.width().saturating_sub(1) as Coord,
        model.height().saturating_sub(1) as Coord,
    ];
    type Node = (Coords, Direction, usize);
    let neighbors = |&(pos, dir, count): &Node| -> Vec<(Cost, Node)> {
        use Direction::*;
        let mut result = Vec::new();
        for d in [East, North, West, South] {
            if d == dir.opposite() {
                continue;
            }
            if d == dir && count >= 3 {
                continue;
            }

            if let Some(p) = d.apply(pos) {
                if let Some(&c) = model.get(p) {
                    let n = if d == dir { count + 1 } else { 1 };
                    result.push((c, (p, d, n)));
                }
            }
        }
        result
    };
    let node_id = |x: &Node| *x;
    let is_goal = |(pos, _, _): &Node| *pos == goal;

    astar((start, Direction::East, 0), node_id, neighbors, is_goal)
        .expect("No path found")
        .into_iter()
        .rev()
        .skip(1)
        .map(|(pos, _, _)| model.get(pos).unwrap())
        .sum()
}
fn part2(input: &str) -> Cost {
    let (_, model) = parse(input).finish().unwrap();
    let start = [0, 0];
    let goal = [
        model.width().saturating_sub(1),
        model.height().saturating_sub(1),
    ];
    type Node = (Coords, Direction, usize);
    let neighbors = |&(pos, dir, count): &Node| -> Vec<(Cost, Node)> {
        use Direction::*;
        let mut result = Vec::new();
        for d in [East, North, West, South] {
            if d == dir.opposite() {
                continue;
            }
            if d == dir && count >= 10 {
                continue;
            }
            if d != dir && count < 4 {
                continue;
            }

            if let Some(p) = d.apply(pos) {
                if let Some(&c) = model.get(p) {
                    let n = if d == dir { count + 1 } else { 1 };
                    result.push((c, (p, d, n)));
                }
            }
        }
        result
    };
    let node_id = |x: &Node| *x;
    let is_goal = |(pos, _, count): &Node| *pos == goal && *count >= 4;

    astar((start, Direction::East, 0), node_id, neighbors, is_goal)
        .expect("No path found")
        .into_iter()
        .rev()
        .skip(1)
        .map(|(pos, _, _)| model.get(pos).unwrap())
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day17 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day17_test.txt");
    const TEST_INPUT_2: &str = include_str!("../data/day17_test_2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 1013);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 94);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(TEST_INPUT_2), 71);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 1215);
    }
}
