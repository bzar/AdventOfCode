use std::ops;

use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_5_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_5_puzzle.txt");

type Number = u64;
type Range = ops::RangeInclusive<Number>;
type Input = (Vec<Range>, Vec<Number>);
type Output = Number;

fn parser<'src>() -> impl Parser<'src, &'src str, Input> {
    let range = text::int(10)
        .then_ignore(just('-'))
        .then(text::int(10))
        .map(|(a, b): (&str, &str)| {
            let start = a.parse().unwrap();
            let end = b.parse().unwrap();
            start..=end
        });
    let ranges = range.separated_by(text::newline()).collect();
    let item = text::int(10).map(|i: &str| i.parse().unwrap());
    let items = item.separated_by(text::newline()).collect();

    ranges
        .then_ignore(text::newline().repeated())
        .then(items)
        .padded()
}

fn part_1((ranges, items): &Input) -> Output {
    items
        .iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count() as Number
}

fn part_2((ranges, _): &Input) -> Output {
    let mut ranges = ranges.clone();
    ranges.sort_by_key(|r| *r.start());

    ranges
        .into_iter()
        .scan(0u64, |progress, range| {
            if *progress <= *range.end() {
                let count = *range.end() - (*progress).max(*range.start()) + 1;
                *progress = *range.end() + 1;
                Some(count)
            } else {
                Some(0)
            }
        })
        .sum()
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
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 14);
    }
}
