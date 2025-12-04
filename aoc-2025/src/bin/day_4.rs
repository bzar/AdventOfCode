use std::collections::HashSet;

use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_4_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_4_puzzle.txt");

type Coord = isize;
type Coords = (Coord, Coord);
type Rolls = HashSet<Coords>;

type Input<'a> = Vec<Vec<bool>>;
type Output = usize;

const ADJACENT: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];
fn parser<'src>() -> impl Parser<'src, &'src str, Input<'src>> {
    let cell = choice((just('@').to(true), just('.').to(false)));
    let line = cell.repeated().collect();
    line.separated_by(text::newline()).collect().padded()
}

fn input_to_rolls(input: &Input) -> Rolls {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (x, y, *cell)))
        .filter_map(|(x, y, cell)| cell.then_some((x as isize, y as isize)))
        .collect()
}

fn part_1(input: &Input) -> Output {
    let rolls = input_to_rolls(input);
    rolls
        .iter()
        .filter(|coords| is_removable(*coords, &rolls))
        .count()
}

fn is_removable(&(x, y): &Coords, rolls: &Rolls) -> bool {
    let mut count = 0;
    for (dx, dy) in ADJACENT {
        if rolls.contains(&(x + dx, y + dy)) {
            count += 1;
        }
        if count >= 4 {
            return false;
        }
    }
    true
}
fn find_removable(rolls: &HashSet<Coords>) -> Vec<Coords> {
    rolls
        .iter()
        .filter(|coords| is_removable(*coords, rolls))
        .copied()
        .collect()
}
fn part_2(input: &Input) -> Output {
    let mut rolls = input_to_rolls(input);

    let initial_count = rolls.len();

    while let removable = find_removable(&rolls)
        && !removable.is_empty()
    {
        for roll in removable {
            rolls.remove(&roll);
        }
    }

    initial_count - rolls.len()
}

fn main() {
    let input = parser().parse(PUZZLE_DATA).unwrap();
    let part_1_result = part_1(&input);
    println!("part 1: {part_1_result}");
    let part_2_result = part_2(&input);
    println!("part 2: {part_2_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 43);
    }
}
