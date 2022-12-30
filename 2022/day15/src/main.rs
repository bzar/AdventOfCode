use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Coord = i64;
type Position = (Coord, Coord);
type Sensor = (Position, Position);
type Sensors = Vec<Sensor>;

fn parse(input: &str) -> nom::IResult<&str, Sensors> {
    fn position(i: &str) -> nom::IResult<&str, Position> {
        preceded(tag("x="), separated_pair(ncc::i64, tag(", y="), ncc::i64))(i)
    }
    let line = preceded(
        tag("Sensor at "),
        separated_pair(position, tag(": closest beacon is at "), position),
    );

    separated_list0(ncc::line_ending, line)(input)
}

fn covered_on_row(((x, y), (bx, by)): &Sensor, row: i64) -> std::ops::Range<Coord> {
    let d = (x.abs_diff(*bx) + y.abs_diff(*by)).saturating_sub(y.abs_diff(row)) as i64;
    if d > 0 {
        x - d..(x + d + 1)
    } else {
        0..0
    }
}
fn merge_ranges(xs: &mut Vec<std::ops::Range<Coord>>) {
    'next: loop {
        for i in 0..xs.len() - 1 {
            for j in i + 1..xs.len() {
                let a = &xs[i];
                let b = &xs[j];
                if a.contains(&b.start)
                    || a.contains(&b.end)
                    || b.contains(&a.start)
                    || b.contains(&a.end)
                    || a.start.abs_diff(b.end) == 0
                    || a.end.abs_diff(b.start) == 0
                {
                    xs[i] = a.start.min(b.start)..a.end.max(b.end);
                    xs.swap_remove(j);
                    continue 'next;
                }
            }
        }
        return;
    }
}
fn part1(sensors: &Sensors, row: i64) -> i64 {
    let mut ranges: Vec<_> = sensors.iter().map(|s| covered_on_row(s, row)).collect();
    merge_ranges(&mut ranges);
    let covered: i64 = ranges.into_iter().map(|r| r.end - r.start).sum();
    covered - 1
}

fn part2(sensors: &Sensors, min: Coord, max: Coord) -> Option<i64> {
    let frequency = |(x, y): Position| x * 4_000_000 + y;
    for y in min..=max {
        let mut x = min;
        while x <= max {
            if let Some(range) = sensors
                .iter()
                .map(|s| covered_on_row(s, y))
                .filter(|r| r.contains(&x))
                .next()
            {
                x = range.end;
            } else {
                return Some(frequency((x, y)));
            }
        }
    }
    None
}

fn main() {
    let (_, sensors) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&sensors, 2_000_000));
    println!(
        "Part 2: {}",
        part2(&sensors, 0, 4_000_000).expect("No solution found")
    );
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, sensors) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&sensors, 10), 26);
    }

    #[test]
    fn test_part1_puzzle() {
        let (_, sensors) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&sensors, 2_000_000), 4861076);
    }
    #[test]
    fn test_part2() {
        let (_, sensors) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&sensors, 0, 20), Some(56000011));
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, sensors) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&sensors, 0, 4_000_000), Some(10649103160102));
    }
}
