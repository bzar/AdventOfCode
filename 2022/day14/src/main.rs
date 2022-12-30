use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::tag, character::complete as ncc, combinator::map, multi::separated_list0,
    sequence::separated_pair, Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Coord = i32;
type Position = (Coord, Coord);
#[derive(Debug, Clone)]
struct Cave {
    floors: HashMap<Coord, Vec<RangeInclusive<Coord>>>,
    walls: HashMap<Coord, Vec<RangeInclusive<Coord>>>,
    sand: HashSet<Position>,
    range: (RangeInclusive<Coord>, RangeInclusive<Coord>),
}

impl Cave {
    fn new(lines: Vec<Vec<Position>>) -> Cave {
        let (x_min, x_max, y_max) = lines
            .iter()
            .flat_map(|xs| xs.iter())
            .fold((Coord::MAX, Coord::MIN, Coord::MIN), |(x_min, x_max, y_max), (x, y)| {
                (x_min.min(*x), x_max.max(*x), y_max.max(*y))
            });
        let range = (x_min..=x_max, Coord::MIN..=y_max);

        let mut floors: HashMap<Coord, Vec<RangeInclusive<Coord>>> = HashMap::new();
        let mut walls: HashMap<Coord, Vec<RangeInclusive<Coord>>> = HashMap::new();
        for line in lines.into_iter() {
            let mut it = line.into_iter();
            if let Some(first) = it.next() {
                it.scan(first, |prev, next| {
                    let segment = (*prev, next);
                    *prev = next;
                    Some(segment)
                })
                .for_each(|((x0, y0), (x1, y1))| {
                    if x0 == x1 {
                        walls
                            .entry(x0)
                            .or_default()
                            .push(if y0 < y1 { y0..=y1 } else { y1..=y0 });
                    } else if y0 == y1 {
                        floors
                            .entry(y0)
                            .or_default()
                            .push(if x0 < x1 { x0..=x1 } else { x1..=x0 });
                    } else {
                        unimplemented!("Diagonal wall")
                    }
                });
            }
        }
        fn merge_ranges(xs: &mut Vec<RangeInclusive<Coord>>) {
            'next: loop {
                for i in 0..xs.len() - 1 {
                    for j in i + 1..xs.len() {
                        let a = &xs[i];
                        let b = &xs[j];
                        if a.contains(b.start())
                            || a.contains(b.end())
                            || b.contains(a.start())
                            || b.contains(a.end())
                            || a.start().abs_diff(*b.end()) <= 1
                            || a.end().abs_diff(*b.start()) <= 1
                        {
                            xs[i] = *(a.start().min(b.start()))..=*(a.end().max(b.end()));
                            xs.swap_remove(j);
                            continue 'next;
                        }
                    }
                }
                return;
            }
        }
        floors.values_mut().for_each(merge_ranges);
        walls.values_mut().for_each(merge_ranges);

        let sand = HashSet::new();
        Cave {
            floors,
            walls,
            sand,
            range,
        }
    }
    fn in_range(&self, (x, y): &Position) -> bool {
        self.range.0.contains(x) && self.range.1.contains(y)
    }
    fn drop_sand(&mut self, drop: &mut Vec<Position>) -> Option<Position> {
        let (mut x, mut y) = drop.pop()?;
        while self.in_range(&(x, y)) {
            let ny = y + 1;
            let floors = self.floors.get(&ny);
            let hit_floor = |x| {
                floors
                    .map(|f| f.iter().any(|f| f.contains(&x)))
                    .unwrap_or(false)
            };
            let hit_wall = |x| {
                self.walls
                    .get(&x)
                    .map(|f| f.iter().any(|f| f.contains(&ny)))
                    .unwrap_or(false)
            };
            let next = [x, x - 1, x + 1]
                .into_iter()
                .filter(|nx| !self.sand.contains(&(*nx, ny)) && !hit_floor(*nx) && !hit_wall(*nx))
                .next();

            if let Some(nx) = next {
                drop.push((x, y));
                (x, y) = (nx, ny);
            } else {
                self.sand.insert((x, y));
                return Some((x, y));
            }
        }

        None
    }
}
fn parse(input: &str) -> nom::IResult<&str, Cave> {
    let line = separated_list0(
        tag(" -> "),
        separated_pair(ncc::i32, ncc::char(','), ncc::i32),
    );
    map(separated_list0(ncc::line_ending, line), Cave::new)(input)
}

fn part1(mut cave: Cave) -> usize {
    let mut drop: Vec<Position> = [(500, 0)].into();
    (0..).map_while(|_| cave.drop_sand(&mut drop)).count()
}

fn part2(mut cave: Cave) -> usize {
    cave.floors
        .entry(cave.range.1.end() + 2)
        .or_default()
        .push(Coord::MIN..=Coord::MAX);

    cave.range = (Coord::MIN..=Coord::MAX, 0..=(cave.range.1.end() + 2));
    let mut drop: Vec<Position> = [(500, 0)].into();
    (0..)
        .map_while(|_| cave.drop_sand(&mut drop))
        .take_while(|pos| *pos != (500, 0))
        .count()
        + 1
}

fn main() {
    let (_, cave) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(cave.clone()));
    println!("Part 2: {}", part2(cave));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, cave) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(cave), 24);
    }

    #[test]
    fn test_part1_puzzle() {
        let (_, cave) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(cave), 828);
    }
    #[test]
    fn test_part2() {
        let (_, cave) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(cave), 93);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, cave) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(cave), 25500);
    }
}
