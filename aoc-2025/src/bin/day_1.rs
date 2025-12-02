use chumsky::prelude::*;

type Amount = i32;
type Dial = i32;
type Count = u32;
const N: Dial = 100;
const DIAL_START: Dial = 50;

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Amount>> {
    let amount = text::int(10).map(|s: &str| s.parse().unwrap());
    let instruction = choice((
        just('L').ignore_then(amount).map(|amount: Amount| -amount),
        just('R').ignore_then(amount),
    ));

    instruction.separated_by(text::newline()).collect().padded()
}

fn rotate(dial: Dial, amount: Dial) -> (Count, Dial) {
    let new_dial = (dial + amount % N + N) % N;
    let rotations = (amount.abs() / N) as Count;
    if dial != 0
        && (new_dial == 0 || amount < 0 && new_dial > dial || amount > 0 && new_dial < dial)
    {
        (rotations + 1, new_dial)
    } else {
        (rotations, new_dial)
    }
}

fn part_1(input: &str) -> Count {
    let amounts = parser().parse(input).unwrap();
    amounts
        .into_iter()
        .fold((0, DIAL_START), |(count, dial), amount| {
            let (_, new_dial) = rotate(dial, amount);
            if new_dial == 0 {
                (count + 1, new_dial)
            } else {
                (count, new_dial)
            }
        })
        .0
}

fn part_2(input: &str) -> Count {
    let amounts = parser().parse(input).unwrap();
    amounts
        .into_iter()
        .fold((0, DIAL_START), |(count, dial), amount| {
            let (increment, new_dial) = rotate(dial, amount);
            (count + increment, new_dial)
        })
        .0
}

fn main() {
    let part_1_result = part_1(include_str!("day_1_puzzle.txt"));
    println!("part 1: {part_1_result}");
    let part_2_result = part_2(include_str!("day_1_puzzle.txt"));
    println!("part 2: {part_2_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = include_str!("day_1_sample.txt");
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn part_2_sample() {
        let input = include_str!("day_1_sample.txt");
        assert_eq!(part_2(input), 6);
    }
}
