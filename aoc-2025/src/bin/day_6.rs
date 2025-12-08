const SAMPLE_DATA: &str = include_str!("day_6_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_6_puzzle.txt");

type Number = u64;
#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Mul,
}
#[derive(Copy, Clone, Debug)]
enum Token {
    Number(Number),
    Operator(Operator),
}
type Input = &'static str;
type Output = Number;

fn part_1(input: &Input) -> Output {
    puzzles(input)
        .map(|puzzle| solve(puzzle_tokens(&puzzle)))
        .sum::<Number>()
}

fn part_2(input: &Input) -> Output {
    puzzles(input)
        .map(|puzzle| solve(puzzle_tokens_2(&puzzle)))
        .sum::<Number>()
}

fn puzzles(worksheet: &str) -> impl Iterator<Item = Vec<String>> {
    let lines: Vec<&str> = worksheet.split('\n').filter(|l| !l.is_empty()).collect();
    let cols = lines[0].len();
    (0..=cols)
        .scan(0usize, move |start, x| {
            if x == cols || lines.iter().all(|line| &line[x..x + 1] == " ") {
                let puzzle =
                    lines
                        .iter()
                        .map(|l| &l[*start..x])
                        .fold(Vec::new(), |mut result, line| {
                            result.push(line.to_owned());
                            result
                        });
                *start = x + 1;
                Some(Some(puzzle))
            } else {
                Some(None)
            }
        })
        .filter_map(|x| x)
}
fn puzzle_tokens(puzzle: &Vec<String>) -> impl Iterator<Item = Token> {
    puzzle.iter().map(|line| {
        if let Ok(number) = line.trim().parse::<Number>() {
            Token::Number(number)
        } else if line.contains('+') {
            Token::Operator(Operator::Add)
        } else if line.contains('*') {
            Token::Operator(Operator::Mul)
        } else {
            panic!("Unexpected input: {}", line);
        }
    })
}
fn puzzle_tokens_2(puzzle: &Vec<String>) -> impl Iterator<Item = Token> {
    let number_lines = &puzzle[..puzzle.len() - 1];
    let operator = match puzzle[puzzle.len() - 1].trim() {
        "+" => Operator::Add,
        "*" => Operator::Mul,
        _ => panic!("Unknown operator"),
    };
    let cols = puzzle[0].len();
    let numbers = (0..cols)
        .map(|col| {
            number_lines
                .iter()
                .map(|line| &line[col..col + 1])
                .collect::<String>()
        })
        .map(|line| Token::Number(line.trim().parse().unwrap()));

    [Token::Operator(operator)].into_iter().chain(numbers)
}

fn solve(tokens: impl Iterator<Item = Token>) -> Number {
    let mut operator = None;
    let mut numbers = Vec::new();

    for t in tokens {
        match t {
            Token::Number(n) => numbers.push(n),
            Token::Operator(o) => operator = Some(o),
        }
    }

    match operator {
        Some(Operator::Add) => numbers.iter().sum(),
        Some(Operator::Mul) => numbers.iter().product(),
        None => panic!("Missing operator"),
    }
}
fn main() {
    let input = PUZZLE_DATA;
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
        let result = puzzles(SAMPLE_DATA)
            .map(|puzzle| solve(puzzle_tokens(&puzzle)))
            .sum::<Number>();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part_2_sample() {
        let result = puzzles(SAMPLE_DATA)
            .map(|puzzle| solve(puzzle_tokens_2(&puzzle)))
            .sum::<Number>();
        assert_eq!(result, 3263827);
    }
}
