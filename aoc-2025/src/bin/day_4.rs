use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_4_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_4_puzzle.txt");

type Coord = isize;
type Coords = (Coord, Coord);

type Input<'a> = Vec<Vec<bool>>;
type Output = usize;

fn parser<'src>() -> impl Parser<'src, &'src str, Input<'src>> {
    let cell = choice((just('@').to(true), just('.').to(false)));
    let line = cell.repeated().collect();
    line.separated_by(text::newline()).collect().padded()
}

fn has_roll(&(x, y): &Coords, input: &Input) -> bool {
    input
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
        .unwrap_or(false)
}

fn rolls(input: &Input) -> impl Iterator<Item = Coords> {
    input.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, cell)| cell.then_some((x as isize, y as isize)))
    })
}

fn adjacent(&(x, y): &Coords, input: &Input) -> impl Iterator<Item = Coords> {
    let y_min = (y - 1).max(0);
    let y_max = (y + 1).min(input.len() as isize - 1);
    let x_min = (x - 1).max(0);
    let x_max = (x + 1).min(input[0].len() as isize - 1); // Assume non-zero rectangle shape
    (y_min..=y_max)
        .flat_map(move |y| (x_min..=x_max).map(move |x| (x, y)))
        .filter(move |(ax, ay)| *ax != x || *ay != y)
}

fn is_removable(coords: &Coords, input: &Input) -> bool {
    let mut count = 0;
    for adj in adjacent(coords, input) {
        if has_roll(&adj, input) {
            count += 1;

            if count >= 4 {
                return false;
            }
        }
    }
    true
}

fn find_removable(input: &Input) -> Vec<Coords> {
    rolls(input)
        .filter(|coords| is_removable(coords, input))
        .collect()
}

fn part_1(input: &Input) -> Output {
    rolls(input)
        .filter(|coords| is_removable(coords, input))
        .count()
}

fn part_2(input: &Input) -> Output {
    let mut map = input.clone();

    while let removable = find_removable(&map)
        && !removable.is_empty()
    {
        for (ax, ay) in removable {
            map[ay as usize][ax as usize] = false;
        }
    }

    rolls(input).count() - rolls(&map).count()
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
