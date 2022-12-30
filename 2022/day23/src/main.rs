use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish,
};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
type Data = Vec<Vec<bool>>;
type Coord = i32;
type Position = (Coord, Coord);
type Elves = HashSet<Position>;
type Order = [Position; 4];
fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data> {
    let line = many1(alt((
        value(false, ncc::char('.')),
        value(true, ncc::char('#')),
    )));
    let data = separated_list1(ncc::line_ending, line);
    all_consuming(delimited(ncc::multispace0, data, ncc::multispace0))(input)
}

fn elves_from_data(data: &Data) -> Elves {
    data.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .filter_map(|(x, y, c)| c.then_some((x as i32, y as i32)))
        .collect()
}
fn elves_nearby((x, y): &Position, elves: &Elves) -> [[bool; 3]; 3] {
    [-1, 0, 1]
        .map(|dy| [-1, 0, 1].map(|dx| (dx == 0 && dy == 0) || elves.contains(&(x + dx, y + dy))))
}
fn elf_plan((x, y): &Position, elves: &Elves, order: &Order) -> Option<Position> {
    let nearby = elves_nearby(&(*x, *y), elves);
    if nearby.iter().flatten().filter(|c| **c).count() == 1 {
        return None;
    }
    for (dx, dy) in order {
        let clear = if *dx == 0 {
            nearby[(dy + 1) as usize].iter().all(|x| !x)
        } else if *dy == 0 {
            nearby.iter().all(|row| !row[(dx + 1) as usize])
        } else {
            unreachable!()
        };

        if clear {
            return Some((x + dx, y + dy));
        }
    }
    None
}
fn play_round(elves: &mut Elves, order: &mut Order) -> bool {
    thread_local!(static THREAD_PLANS: RefCell<HashMap<Position, Option<Position>>> = RefCell::new(HashMap::with_capacity(256)));
    THREAD_PLANS.with(|plans_cell| {
        let mut plans = plans_cell.borrow_mut();
        plans.clear();
        elves
            .iter()
            .filter_map(|pos| elf_plan(pos, elves, order).map(|plan| (pos, plan)))
            .for_each(|(pos, plan)| {
                plans
                    .entry(plan)
                    .and_modify(|e| *e = None)
                    .or_insert(Some(*pos));
            });
        if plans.values().all(|x| x.is_none()) {
            return true;
        }
        for (to, from) in plans.iter() {
            if let Some(from) = from {
                elves.remove(from);
                elves.insert(*to);
            }
        }

        order.rotate_left(1);
        false
    })
}
fn calculate_progress(elves: &Elves) -> usize {
    let min_x = *elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *elves.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *elves.iter().map(|(_, y)| y).max().unwrap();
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|pos| !elves.contains(pos))
        .count()
}
fn part1(data: &Data) -> usize {
    let mut elves = elves_from_data(data);
    let mut order = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for _ in 0..10 {
        play_round(&mut elves, &mut order);
    }
    calculate_progress(&elves)
}

fn part2(data: &Data) -> usize {
    let mut elves = elves_from_data(data);
    let mut order = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    (2..)
        .take_while(|_| !play_round(&mut elves, &mut order))
        .last()
        .unwrap_or(0)
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
        assert_eq!(part1(&data), 110);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 4162);
    }
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
}
