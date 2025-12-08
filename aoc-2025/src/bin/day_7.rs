use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_7_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_7_puzzle.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Start,
    Empty,
    Splitter,
}
type Input = Vec<Vec<Token>>;
type Output = u64;

fn parser<'src>() -> impl Parser<'src, &'src str, Input> {
    let token = choice((
        just('S').to(Token::Start),
        just('.').to(Token::Empty),
        just('^').to(Token::Splitter),
    ));
    let line = token.repeated().collect().then_ignore(text::newline());
    line.repeated().collect()
}

fn split_beams<'a>(beams: &mut Vec<Output>, lines: impl Iterator<Item = &'a Vec<Token>>) -> Output {
    let mut count = 0;
    for line in lines {
        let split_beams = beams
            .clone()
            .into_iter()
            .zip(line.iter())
            .enumerate()
            .filter_map(|(i, (b, t))| (b > 0 && *t == Token::Splitter).then_some(i));

        for split_index in split_beams {
            if split_index > 0 {
                beams[split_index - 1] += beams[split_index];
            }
            if split_index < beams.len() - 1 {
                beams[split_index + 1] += beams[split_index];
            }
            beams[split_index] = 0;

            count += 1;
        }
    }

    count
}
fn beams_and_lines(input: &Input) -> (Vec<Output>, impl Iterator<Item = &Vec<Token>>) {
    let mut lines = input.iter();
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    let start = first_line
        .iter()
        .position(|t| *t == Token::Start)
        .expect("No start on first line!");
    let mut beams = Vec::new();
    beams.resize(width, 0);
    beams[start] = 1;

    (beams, lines)
}
fn part_1(input: &Input) -> Output {
    let (mut beams, lines) = beams_and_lines(input);
    split_beams(&mut beams, lines)
}

fn part_2(input: &Input) -> Output {
    let (mut beams, lines) = beams_and_lines(input);
    split_beams(&mut beams, lines);
    beams.iter().sum()
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
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 40);
    }
}
