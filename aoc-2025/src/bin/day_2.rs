use chumsky::prelude::*;

type Id = u64;
type Range = (Id, Id);

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Range>> {
    let id = text::int(10).map(|s: &str| s.parse().unwrap());
    let range = id.then_ignore(just('-')).then(id);
    range.separated_by(just(',')).collect().padded()
}

fn is_invalid(id: &Id) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % 2 == 1 {
        return false;
    }

    let scale = (10 as Id).pow(num_digits / 2);
    return id / scale == id % scale;
}

fn part_1(input: &str) -> Id {
    let ranges = parser().parse(input).unwrap();
    ranges
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(is_invalid)
        .sum()
}

fn is_invalid_2(id: &Id) -> bool {
    let num_digits = id.ilog10() + 1;

    (1..=num_digits / 2)
        .filter(|n| num_digits % n == 0)
        .any(|n| {
            let groups = num_digits / n;
            let scale = (10 as Id).pow(n);
            let mut values = (0..groups).map(|g| id / scale.pow(g) % scale);
            let first = values.next().unwrap();
            values.all(|v| v == first)
        })
}

fn part_2(input: &str) -> Id {
    let ranges = parser().parse(input).unwrap();
    ranges
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(is_invalid_2)
        .sum()
}

fn main() {
    let part_1_result = part_1(include_str!("day_2_puzzle.txt"));
    println!("part 1: {part_1_result}");
    let part_2_result = part_2(include_str!("day_2_puzzle.txt"));
    println!("part 2: {part_2_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = include_str!("day_2_sample.txt");
        assert_eq!(part_1(input), 1227775554);
    }

    #[test]
    fn part_2_ids() {
        assert!(is_invalid_2(&12341234));
        assert!(is_invalid_2(&123123123));
        assert!(is_invalid_2(&1212121212));
        assert!(is_invalid_2(&1111111));
    }

    #[test]
    fn part_2_sample() {
        let input = include_str!("day_2_sample.txt");
        assert_eq!(part_2(input), 4174379265);
    }
}
