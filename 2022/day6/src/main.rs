use itertools::Itertools;

fn first_n_unique(n: usize, input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(n)
        .enumerate()
        .find_map(|(i, xs)| xs.iter().all_unique().then_some(i + n))
}

fn part1(input: &str) -> usize {
    first_n_unique(4, input).expect("Start of packet not found")
}

fn part2(input: &str) -> usize {
    first_n_unique(14, input).expect("Start of message not found")
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
}
