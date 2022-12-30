use nom::{
    character::complete as ncc,
    multi::separated_list1,
    sequence::delimited,
    combinator::all_consuming,
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Numbers = Vec<i64>;
fn parse(input: &str) -> nom::IResult<&str, Numbers> {
    all_consuming(delimited(ncc::multispace0, separated_list1(ncc::line_ending, ncc::i64), ncc::multispace0))(input)
}

fn rotate<T>(numbers: &mut Vec<T>, at: usize, delta: i64) {
    let n = numbers.len() as i64;
    // determine preceding index in array without item to rotate
    let after = if at == 0 { n - 2 } else { at as i64 - 1 };
    // calculate shifted index for preceding item
    let after = (after + delta) % (n - 1);
    // handle negative indices
    let after = if after < 0 { after + (n - 1) } else { after };
    // translate index to original array
    let end = after as usize + 1; 

    // determine slice to rotate
    let slice = if end > at {
        &mut numbers[at..=end]
    } else if end < at {
        &mut numbers[end..=at]
    } else {
        return
    };

    // determine type of rotation needed
    if delta > 0 {
        if end > at {
            slice.rotate_left(1);
        } else {
            slice.rotate_right(1);
        }
    } else if delta < 0 {
        if end < at {
            slice.rotate_right(1);
        } else {
            slice.rotate_left(1);
        }
    }
}

fn part1(numbers: &Numbers) -> i64 {
    let mut buffer: Vec<_> = (0..numbers.len()).collect();
    for i in 0..numbers.len() {
        let at = buffer.iter().enumerate().find_map(|(k, j)| (i == *j).then_some(k)).unwrap();
        rotate(&mut buffer, at, numbers[i]);
    }
    let result: Vec<_> = buffer.into_iter().map(|i| numbers[i]).collect();
    let zero = result.iter().enumerate().find_map(|(i, n)| (*n == 0).then_some(i)).unwrap();
    
    result.iter().cycle().skip(zero).step_by(1000).skip(1).take(3).sum()
}

fn part2(numbers: &Numbers) -> i64 {
    const KEY: i64 = 811589153;
    let numbers: Vec<_> = numbers.iter().map(|n| n * KEY).collect();
    let mut buffer: Vec<_> = (0..numbers.len()).collect();
    for _ in 0..10 {
        for i in 0..numbers.len() {
            let at = buffer.iter().enumerate().find_map(|(k, j)| (i == *j).then_some(k)).unwrap();
            rotate(&mut buffer, at, numbers[i]);
        }
    }
    let result: Vec<_> = buffer.into_iter().map(|i| numbers[i]).collect();
    let zero = result.iter().enumerate().find_map(|(i, n)| (*n == 0).then_some(i)).unwrap();
    
    result.iter().cycle().skip(zero).step_by(1000).skip(1).take(3).sum()
}

fn main() {
    let (_, numbers) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_rotate() {
        let mut numbers = vec![1, 2, 3];
        rotate(&mut numbers, 0, 1);
        assert_eq!(numbers, vec![2, 1, 3]);
        rotate(&mut numbers, 1, 1);
        assert_eq!(numbers, vec![2, 3, 1]);
        rotate(&mut numbers, 2, 1);
        assert_eq!(numbers, vec![2, 1, 3]);
        rotate(&mut numbers, 1, 2);
        assert_eq!(numbers, vec![2, 1, 3]);
        rotate(&mut numbers, 0, -1);
        assert_eq!(numbers, vec![1, 2, 3]);
        rotate(&mut numbers, 2, -2);
        assert_eq!(numbers, vec![1, 2, 3]);
        rotate(&mut numbers, 0, 5);
        assert_eq!(numbers, vec![2, 1, 3]);
        rotate(&mut numbers, 2, -5);
        assert_eq!(numbers, vec![2, 3, 1]);
        rotate(&mut numbers, 1, 10);
        assert_eq!(numbers, vec![2, 3, 1]);
        rotate(&mut numbers, 1, 9);
        assert_eq!(numbers, vec![2, 1, 3]);
    }
    #[test]
    fn test_part1() {
        let (_, numbers) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&numbers), 3);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, numbers) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&numbers), 2215);
    }
    #[test]
    fn test_part2() {
        let (_, numbers) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&numbers), 1623178306);
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, numbers) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&numbers), 8927480683);
    }
}




