use nom::{
    branch::alt, character::complete as ncc, combinator::value, multi::separated_list0,
    sequence::separated_pair,
};
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}
type Moves = Vec<(Move, u32)>;

impl From<Move> for (i32, i32) {
    fn from(m: Move) -> Self {
        match m {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Up => (0, -1),
            Move::Down => (0, 1),
        }
    }
}

fn parse(input: &str) -> nom::IResult<&str, Moves> {
    let direction = alt((
        value(Move::Up, ncc::char('U')),
        value(Move::Down, ncc::char('D')),
        value(Move::Left, ncc::char('L')),
        value(Move::Right, ncc::char('R')),
    ));
    separated_list0(
        ncc::line_ending,
        separated_pair(direction, ncc::space1, ncc::u32),
    )(input)
}

fn follow((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    if hx.abs_diff(tx) <= 1 && hy.abs_diff(ty) <= 1 {
        (tx, ty)
    } else {
        (tx + (hx - tx).signum(), ty + (hy - ty).signum())
    }
}

fn tail_positions<const PARTS: usize>(moves: &Moves) -> usize {
    moves
        .into_iter()
        .flat_map(|(m, n)| (0..*n).map(move |_| (*m).into()))
        .scan([(0, 0); PARTS], |state, (dx, dy)| {
            state[0] = (state[0].0 + dx, state[0].1 + dy);
            let head = state[0];
            state
                .iter_mut()
                .fold(head, |h: (i32, i32), x| {
                    *x = follow(h, *x);
                    *x
                })
                .into()
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part1(moves: &Moves) -> usize {
    tail_positions::<2>(moves)
}

fn part2(moves: &Moves) -> usize {
    tail_positions::<10>(moves)
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let (_, moves) = parse(&input).expect("Error parsing input");
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let (_, moves) = parse("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2").unwrap();
        assert_eq!(part1(&moves), 13);
    }
    #[test]
    fn test_part2() {
        let (_, moves) = parse("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap();
        assert_eq!(part2(&moves), 36);
    }
}
