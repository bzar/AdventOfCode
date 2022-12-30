use nom::{
    character::complete as ncc,
    multi::many0,
    sequence::delimited,
    branch::alt,
    combinator::{value, all_consuming},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Row = u16;
type Piece = [Row; 4];

const EMPTY_ROW: Row = 0b1000000011111111;
const NUM_PIECES: usize = 5;
const PIECE_HEIGHT: [usize; NUM_PIECES] = [1, 3, 3, 4, 2];
const PIECES: [Piece; NUM_PIECES] = [
    [0b0001111000000000, 0, 0, 0],
    [0b0000100000000000,
     0b0001110000000000,
     0b0000100000000000, 0],
    [0b0000010000000000,
     0b0000010000000000,
     0b0001110000000000, 0],
    [0b0001000000000000,
     0b0001000000000000,
     0b0001000000000000,
     0b0001000000000000],
    [0b0001100000000000,
     0b0001100000000000, 0, 0],
];

#[derive(Copy, Clone, Debug)]
struct Rock {
    piece: Piece,
    altitude: usize
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right
}
type Moves = Vec<Direction>;
type State = Vec<Row>;

fn parse(input: &str) -> nom::IResult<&str, Moves> {
    let m = alt((value(Direction::Left, ncc::char('<')), value(Direction::Right, ncc::char('>'))));
    all_consuming(delimited(ncc::multispace0, many0(m), ncc::multispace0))(input)
}

fn generate_rock(n: usize, altitude: usize) -> Rock {
    let i = n % PIECES.len();
    Rock { piece:PIECES[i], altitude: altitude + PIECE_HEIGHT[i]}
}
fn move_rock(rock: &mut Rock, state: &State, direction: Direction) {
    let piece = match direction {
        Direction::Left => rock.piece.map(|row| row << 1),
        Direction::Right => rock.piece.map(|row| row >> 1),
    };
    let hit_wall = piece.iter().enumerate()
        .filter(|(i, r)| **r != 0 && rock.altitude >= *i)
        .map(|(i, p)| (rock.altitude - i, p))
        .any(|(h, p)| state.get(h).unwrap_or(&EMPTY_ROW) & p != 0);

    if hit_wall {
        return;
    }
    rock.piece = piece;
}
fn drop_rock(rock: &mut Rock, state: &State) -> bool {
    if rock.altitude == 0 {
        return true;
    }
    let altitude = rock.altitude - 1;
    
    let hit_wall = rock.piece.iter().enumerate()
        .filter(|(_, r)| **r != 0)
        .map(|(i, p)| (altitude - i, p))
        .any(|(h, p)| state.get(h).map(|row| row & p != 0).unwrap_or(false));

    if hit_wall {
        return true;
    }

    rock.altitude = altitude;
    false
}

fn play_rock(i: usize, state: &mut State, moves: &mut impl Iterator<Item=Direction>) {
    let mut rock = generate_rock(i, state.len() + 2);
    loop {
        if let Some(direction) = moves.next() {
            move_rock(&mut rock, &state, direction);
        }
        if drop_rock(&mut rock, state) {
            if rock.altitude + 1 > state.len() {
                state.resize(rock.altitude + 1, EMPTY_ROW);
            }
            for (i, row) in rock.piece.iter().enumerate().filter(|(i, row)| **row != 0 && rock.altitude >= *i) {
                let h = rock.altitude - i;
                state[h] |= row;
            }
            break;
        }
    }
}

fn play(moves: &Moves, n: usize) -> usize {
    let mut state = Vec::new();
    let mut moves = moves.iter().copied().cycle();
    let mut checksums = Vec::new();
    const N: usize = 1024;
    for i in 0..n {
        play_rock(i, &mut state, &mut moves);
        if i > N {
            let checksum: Vec<Row> = state.iter().rev().take(N).copied().collect();
            for (j, l) in checksums.iter().filter_map(|(i, cs, l)| (*cs == checksum).then_some((i, l))) {
                if i - j > 10 {
                    let repeat = i - j;
                    let left = n - i;
                    let repeat_count = left / repeat;
                    let repeat_size = state.len() - l;
                    let remainder = left % repeat - 1;
                    dbg!(repeat, left, repeat_count, repeat_size, remainder);
                    for k in 0..remainder {
                        play_rock(i+1+k, &mut state, &mut moves);
                    }
                    let size = state.len() + repeat_count * repeat_size;
                    return size;
                }
            }
            checksums.push((i, checksum, state.len()));
        }
    }
    state.len()
}

fn part1(moves: &Moves) -> usize {
    play(moves, 2022)
}

fn part2(moves: &Moves) -> usize {
    play(moves, 1_000_000_000_000)
}

fn main() {
    let (_, moves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, moves) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&moves), 3068);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, moves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&moves), 3100);
    }
    #[test]
    fn test_part2() {
        let (_, moves) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&moves), 1_514_285_714_288);
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, moves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&moves), 1_540_634_005_751);
    }
}

