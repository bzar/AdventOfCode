use nom::FindSubstring;

const PUZZLE_INPUT: &str = include_str!("../data/day01.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .unwrap();
            first * 10 + last
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    let digit_names = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    input
        .lines()
        .map(|line| {
            let first = {
                let mut i = None;
                let mut value = None;
                for (name, digit) in digit_names.iter() {
                    if let Some(index) = line.find_substring(name) {
                        if let Some(prev) = i {
                            if index < prev {
                                i = Some(index);
                                value = Some(digit);
                            }
                        } else {
                            i = Some(index);
                            value = Some(digit);
                        }
                    }
                }
                value.unwrap()
            };
            let last = {
                let mut i = None;
                let mut value = None;
                let line_rev: &str = &line.chars().rev().collect::<String>();
                for (name, digit) in digit_names.iter() {
                    if let Some(index) =
                        line_rev.find_substring(&name.chars().rev().collect::<String>())
                    {
                        if let Some(prev) = i {
                            if index < prev {
                                i = Some(index);
                                value = Some(digit);
                            }
                        } else {
                            i = Some(index);
                            value = Some(digit);
                        }
                    }
                }
                value.unwrap()
            };

            first * 10 + last
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day01 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day01_test.txt");
    const TEST_INPUT_2: &str = include_str!("../data/day01_test_2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 142);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 56397);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 281);
    }
    #[test]
    fn test_part2_custom() {
        assert_eq!(part2("1eighthree"), 13);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 55701);
    }
}
