use aoc_2023::{Coord, RectMap};
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::terminated,
    Finish,
};
use std::collections::HashMap;
const PUZZLE_INPUT: &str = include_str!("../data/day14.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Ball,
    Cube,
}
type Model = RectMap<Tile>;

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_tile = alt((
        value(Tile::Empty, ncc::char('.')),
        value(Tile::Ball, ncc::char('O')),
        value(Tile::Cube, ncc::char('#')),
    ));
    all_consuming(map(
        many1(terminated(many1(parse_tile), ncc::line_ending)),
        RectMap::new,
    ))(input)
}

fn part1(input: &str) -> Coord {
    let (_, model) = parse(input).finish().unwrap();
    model
        .columns()
        .flat_map(|column| {
            column.enumerate().scan(None, |dst, (y, tile)| {
                match (tile, dst.clone()) {
                    (Tile::Empty, None) => {
                        *dst = Some(y as Coord);
                        0
                    }
                    (Tile::Cube, _) => {
                        *dst = None;
                        0
                    }
                    (Tile::Ball, None) => model.height() - y as Coord,
                    (Tile::Ball, Some(dy)) => {
                        *dst = Some(dy + 1);
                        model.height() - dy
                    }
                    _ => 0,
                }
                .into()
            })
        })
        .sum()
}
fn calculate_load(model: &Model) -> Coord {
    model
        .cells()
        .filter_map(|([_, y], c)| (*c == Tile::Ball).then_some(model.height() - y))
        .sum()
}
fn tilt_north(model: &mut Model) {
    for x in 0..model.width() {
        let mut dst = None;
        for y in 0..model.height() {
            let tile = model.get([x, y]).unwrap();
            match (tile, dst.clone()) {
                (Tile::Empty, None) => {
                    dst = Some(y);
                }
                (Tile::Cube, _) => {
                    dst = None;
                }
                (Tile::Ball, Some(dy)) => {
                    *model.get_mut([x, y]).unwrap() = Tile::Empty;
                    *model.get_mut([x, dy]).unwrap() = Tile::Ball;
                    dst = Some(dy + 1);
                }
                _ => (),
            }
        }
    }
}
fn tilt_south(model: &mut Model) {
    for x in 0..model.width() {
        let mut dst = None;
        let height = model.height();
        for y in (0..height).map(|y| height - y - 1) {
            let tile = model.get([x, y]).unwrap();
            match (tile, dst.clone()) {
                (Tile::Empty, None) => {
                    dst = Some(y);
                }
                (Tile::Cube, _) => {
                    dst = None;
                }
                (Tile::Ball, Some(dy)) => {
                    *model.get_mut([x, y]).unwrap() = Tile::Empty;
                    *model.get_mut([x, dy]).unwrap() = Tile::Ball;
                    if dy > 0 {
                        dst = Some(dy - 1);
                    }
                }
                _ => (),
            }
        }
    }
}
fn tilt_west(model: &mut Model) {
    for y in 0..model.height() {
        let mut dst = None;
        for x in 0..model.width() {
            let tile = model.get([x, y]).unwrap();
            match (tile, dst.clone()) {
                (Tile::Empty, None) => {
                    dst = Some(x);
                }
                (Tile::Cube, _) => {
                    dst = None;
                }
                (Tile::Ball, Some(dx)) => {
                    *model.get_mut([x, y]).unwrap() = Tile::Empty;
                    *model.get_mut([dx, y]).unwrap() = Tile::Ball;
                    dst = Some(dx + 1);
                }
                _ => (),
            }
        }
    }
}
fn tilt_east(model: &mut Model) {
    for y in 0..model.height() {
        let mut dst = None;
        let width = model.width();
        for x in (0..width).map(|x| width - x - 1) {
            let tile = model.get([x, y]).unwrap();
            match (tile, dst.clone()) {
                (Tile::Empty, None) => {
                    dst = Some(x);
                }
                (Tile::Cube, _) => {
                    dst = None;
                }
                (Tile::Ball, Some(dx)) => {
                    *model.get_mut([x, y]).unwrap() = Tile::Empty;
                    *model.get_mut([dx, y]).unwrap() = Tile::Ball;
                    if dx > 0 {
                        dst = Some(dx - 1);
                    }
                }
                _ => (),
            }
        }
    }
}
fn part2(input: &str) -> Coord {
    let (_, mut model) = parse(input).finish().unwrap();

    let mut previous = HashMap::new();
    const N: u64 = 1000000000;
    for i in 0..N {
        let load = calculate_load(&model);
        if let Some((prev, prev_model)) = previous.get(&load) {
            if *prev_model == model {
                let cycle = i - prev;
                let offset = prev;
                let cycles = (N - offset) / cycle;
                let after_cycle = N - offset - cycles * cycle;

                for _ in 0..after_cycle {
                    tilt_north(&mut model);
                    tilt_west(&mut model);
                    tilt_south(&mut model);
                    tilt_east(&mut model);
                }
                return calculate_load(&model);
            }
        }

        previous.insert(load, (i, model.clone()));
        tilt_north(&mut model);
        tilt_west(&mut model);
        tilt_south(&mut model);
        tilt_east(&mut model);
    }
    return calculate_load(&model);
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day14 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day14_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 106378);
    }
    #[test]
    fn test_tilts() {
        let (_, mut model) = parse(TEST_INPUT).finish().unwrap();
        tilt_north(&mut model);
        tilt_west(&mut model);
        tilt_south(&mut model);
        tilt_east(&mut model);

        let after_one_cycle_input = ".....#....\n\
                                     ....#...O#\n\
                                     ...OO##...\n\
                                     .OO#......\n\
                                     .....OOO#.\n\
                                     .O#...O#.#\n\
                                     ....O#....\n\
                                     ......OOOO\n\
                                     #...O###..\n\
                                     #..OO#....\n";

        let (_, after_one_cycle) = parse(after_one_cycle_input).finish().unwrap();
        assert_eq!(model, after_one_cycle);

        tilt_north(&mut model);
        tilt_west(&mut model);
        tilt_south(&mut model);
        tilt_east(&mut model);
        let after_two_cycle_input = ".....#....\n\
                                     ....#...O#\n\
                                     .....##...\n\
                                     ..O#......\n\
                                     .....OOO#.\n\
                                     .O#...O#.#\n\
                                     ....O#...O\n\
                                     .......OOO\n\
                                     #..OO###..\n\
                                     #.OOO#...O\n";

        let (_, after_two_cycle) = parse(after_two_cycle_input).finish().unwrap();
        assert_eq!(model, after_two_cycle);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 90795);
    }
}
