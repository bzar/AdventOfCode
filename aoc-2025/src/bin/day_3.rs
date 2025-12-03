use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_3_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_3_puzzle.txt");
type Input<'a> = Vec<&'a str>;
type Output = u64;

fn parser<'src>() -> impl Parser<'src, &'src str, Input<'src>> {
    let line = text::digits(10).to_slice();
    line.separated_by(text::newline()).collect().padded()
}

fn maximum_joltage<const N: usize>(bank: &str) -> Output {
    (0..N)
        .scan(0usize, |start, n| {
            let end: usize = bank.len() - (N - n - 1);
            let chars: &str = &bank[*start..end];
            let first_max = |(index, max), (i, c)| {
                if c > max { (i, c) } else { (index, max) }
            };
            let (index, max) = chars.chars().enumerate().fold((0, '0'), first_max);
            *start = *start + index + 1;
            Some(max)
        })
        .fold(0, |sum, c| sum * 10 + c.to_digit(10).unwrap() as Output)
}

fn part_1(input: &Input) -> Output {
    input.iter().copied().map(maximum_joltage::<2>).sum()
}

fn part_2(input: &Input) -> Output {
    input.iter().copied().map(maximum_joltage::<12>).sum()
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
        assert_eq!(part_1(&input), 357);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 3121910778619);
    }
}
