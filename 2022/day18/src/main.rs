use nom::{
    character::complete as ncc,
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    Finish,
};
use std::collections::HashSet;

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Coord = i32;
type Cube = (Coord, Coord, Coord);
type Cubes = Vec<Cube>;

fn parse(input: &str) -> nom::IResult<&str, Cubes> {
    let cube = tuple((
        ncc::i32,
        preceded(ncc::char(','), ncc::i32),
        preceded(ncc::char(','), ncc::i32),
    ));
    all_consuming(delimited(
        ncc::multispace0,
        separated_list0(ncc::line_ending, cube),
        ncc::multispace0,
    ))(input)
}

fn is_adjacent((x0, y0, z0): Cube, (x1, y1, z1): Cube) -> bool {
    x0.abs_diff(x1) + y0.abs_diff(y1) + z0.abs_diff(z1) == 1
}
fn part1(cubes: &Cubes) -> i32 {
    cubes
        .iter()
        .enumerate()
        .map(|(i, cube)| {
            6 - 2 * cubes
                .iter()
                .skip(i + 1)
                .filter(|c| is_adjacent(*cube, **c))
                .count() as i32
        })
        .sum()
}

fn neighbors(&(x, y, z): &Cube) -> [Cube; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}
fn find_air(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let ((x_min, y_min, z_min), (x_max, y_max, z_max)) = cubes.iter().fold(
        (
            (Coord::MAX, Coord::MAX, Coord::MAX),
            (Coord::MIN, Coord::MIN, Coord::MIN),
        ),
        |((x_min, y_min, z_min), (x_max, y_max, z_max)), &(x, y, z)| {
            (
                (x_min.min(x), y_min.min(y), z_min.min(z)),
                (x_max.max(x), y_max.max(y), z_max.max(z)),
            )
        },
    );

    let mut visited = HashSet::new();
    let mut stack: Vec<_> = [(x_min - 1, y_min - 1, z_min - 1)].into();
    while let Some((nx, ny, nz)) = stack.pop() {
        if nx < x_min - 1
            || nx > x_max + 1
            || ny < y_min - 1
            || ny > y_max + 1
            || nz < z_min - 1
            || nz > z_max + 1
        {
            continue;
        }
        visited.insert((nx, ny, nz));
        stack.extend(
            neighbors(&(nx, ny, nz))
                .into_iter()
                .filter(|n| !cubes.contains(n) && !visited.contains(n)),
        );
    }
    visited
}
fn part2(cubes: &Cubes) -> usize {
    let cubes: HashSet<Cube> = cubes.iter().copied().collect();
    let air = find_air(&cubes);
    cubes
        .iter()
        .flat_map(|c| neighbors(c))
        .filter(|n| air.contains(n))
        .count()
}

fn main() {
    let (_, cubes) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&cubes));
    println!("Part 2: {}", part2(&cubes));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_trivial() {
        let (_, cubes) = parse("1,1,1\n2,1,1").finish().expect("Parse error");
        assert_eq!(part1(&cubes), 10);
    }
    #[test]
    fn test_part1() {
        let (_, cubes) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&cubes), 64);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, cubes) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&cubes), 3412);
    }
    #[test]
    fn test_part2() {
        let (_, cubes) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&cubes), 58);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, cubes) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&cubes), 2018);
    }
}
