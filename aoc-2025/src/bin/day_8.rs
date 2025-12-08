use std::collections::HashSet;

use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_8_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_8_puzzle.txt");

type Coord = u64;
type Junction = (Coord, Coord, Coord);
type Input = Vec<Junction>;
type Output = u64;

fn parser<'src>() -> impl Parser<'src, &'src str, Input> {
    let coord = text::int(10).map(|s: &str| s.parse::<Coord>().unwrap());
    let junction = coord
        .then_ignore(just(','))
        .then(coord)
        .then_ignore(just(','))
        .then(coord)
        .map(|((x, y), z)| (x, y, z));
    junction.separated_by(text::newline()).collect().padded()
}

fn distance_coord(a: &Coord, b: &Coord) -> Coord {
    if a > b { a - b } else { b - a }
}
fn distance_sq((x0, y0, z0): &Junction, (x1, y1, z1): &Junction) -> Coord {
    let (dx, dy, dz) = (
        distance_coord(x0, x1),
        distance_coord(y0, y1),
        distance_coord(z0, z1),
    );
    dx * dx + dy * dy + dz * dz
}
fn part_1(input: &Input, num_connections: usize) -> Output {
    let n = input.len();
    let pairs = (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j)));
    let mut distances: Vec<_> = pairs
        .map(|(i, j)| (distance_sq(&input[i], &input[j]), (i, j)))
        .collect();
    distances.sort();

    let mut circuits: Vec<_> = (0..n)
        .map(|i| [i].into_iter().collect::<HashSet<_>>())
        .collect();
    let mut junction_circuits: Vec<_> = (0..n).map(|i| i).collect();
    let mut connections = 0;

    for (_distance, (i, j)) in distances {
        connections += 1;
        if connections > num_connections {
            break;
        }
        let jci = junction_circuits[i];
        let jcj = junction_circuits[j];

        if jci == jcj {
            continue; // Same circuit
        }

        let (src, dst) = if circuits[jci].len() > circuits[jcj].len() {
            (jcj, jci)
        } else {
            (jci, jcj)
        };

        let mut moved = HashSet::new();
        std::mem::swap(&mut moved, &mut circuits[src]);
        for k in moved {
            circuits[dst].insert(k);
            junction_circuits[k] = dst;
        }
    }

    circuits.sort_by_key(|c| c.len());
    circuits
        .iter()
        .rev()
        .take(3)
        .map(|c| c.len())
        .product::<usize>() as Coord
}

fn part_2(input: &Input) -> Output {
    let n = input.len();
    let pairs = (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j)));
    let mut distances: Vec<_> = pairs
        .map(|(i, j)| (distance_sq(&input[i], &input[j]), (i, j)))
        .collect();
    distances.sort();

    let mut circuits: Vec<_> = (0..n)
        .map(|i| [i].into_iter().collect::<HashSet<_>>())
        .collect();
    let mut junction_circuits: Vec<_> = (0..n).map(|i| i).collect();

    for (_distance, (i, j)) in distances {
        let jci = junction_circuits[i];
        let jcj = junction_circuits[j];

        if jci == jcj {
            continue; // Same circuit
        }

        let (src, dst) = if circuits[jci].len() > circuits[jcj].len() {
            (jcj, jci)
        } else {
            (jci, jcj)
        };

        let mut moved = HashSet::new();
        std::mem::swap(&mut moved, &mut circuits[src]);
        for k in moved {
            circuits[dst].insert(k);
            junction_circuits[k] = dst;
        }

        if circuits.iter().filter(|c| !c.is_empty()).count() == 1 {
            return input[i].0 * input[j].0;
        }
    }
    panic!("Did not finish!")
}

fn main() {
    let input = parser().parse(PUZZLE_DATA).unwrap();
    let part_1_result = part_1(&input, 1000);
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
        assert_eq!(part_1(&input, 10), 40);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 25272);
    }
}
