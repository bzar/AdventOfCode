use std::collections::HashMap;
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{map, all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
//   01
//   2
//  43
//  5
const PUZZLE_FACES: [(Position, [(usize, Direction); 4]); 6] = [
    ((50, 0), [
     (1, Direction::East),
     (2, Direction::South),
     (4, Direction::East),
     (5, Direction::East),
    ]),
    ((100, 0), [
     (3, Direction::West),
     (2, Direction::West),
     (0, Direction::West),
     (5, Direction::North),
    ]),
    ((50, 50), [
     (1, Direction::North),
     (3, Direction::South),
     (4, Direction::South),
     (0, Direction::North),
    ]),
    ((50, 100), [
     (1, Direction::West),
     (5, Direction::West),
     (4, Direction::West),
     (2, Direction::North),
    ]),
    ((0, 100), [
     (3, Direction::East),
     (5, Direction::South),
     (0, Direction::East),
     (2, Direction::East),
    ]),
    ((0, 150), [
     (3, Direction::North),
     (1, Direction::South),
     (0, Direction::South),
     (4, Direction::North),
    ]),
    ];
const PUZZLE_SIZE: usize = 50;


type Coord = usize;
type Position = (Coord, Coord);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction { East = 0, South = 1, West = 2, North = 3 }
struct Character {
    position: Position,
    heading: Direction
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Floor,
    Empty
}

type Map = Vec<Vec<Cell>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Node {
    Wall, 
    Floor([(Position, Direction); 4]) // Direction-indexed neighbors
}
type Graph = HashMap<Position, Node>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Advance(u32),
    Left,
    Right
}
type Instructions = Vec<Instruction>;
type Data = (Map, Instructions);

impl Character {
    fn advance(&mut self, graph: &Graph) {
        if let Some(Node::Floor(neighbors)) = graph.get(&self.position) {
            (self.position, self.heading) = neighbors[self.heading as usize];
        } else {
            unreachable!("Inside a wall!")
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }
    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}
fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data> {
    use Cell::*;
    let map_line = many1(alt((
        value(Wall, ncc::char('#')),
        value(Floor, ncc::char('.')),
        value(Empty, ncc::char(' ')),
    )));
    let map_data = separated_list1(ncc::line_ending, map_line);
    let instructions = many1(alt((
                map(ncc::u32, Instruction::Advance),
                value(Instruction::Left, ncc::char('L')),
                value(Instruction::Right, ncc::char('R')),
                )));
    let data = separated_pair(map_data, ncc::multispace1, instructions);
    all_consuming(terminated(data, ncc::multispace0))(input)
}

fn wrap_neighbor<'a>(map: &'a Map) -> impl Fn(Position, Direction) -> (Position, Direction) + 'a {
    |(x, y): Position, d: Direction| { 
        let (dx, dy): (i32, i32) = match d {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
        };
        let should_wrap = ((x == 0 && dx < 0) || (y == 0 && dy < 0)) 
            || map.get((y as i32 + dy) as usize)
                .map(|row| row.get((x as i32 + dx) as usize))
                .flatten()
                .map(|c| c == &Cell::Empty)
                .unwrap_or(true);
        if should_wrap {
            use Cell::Empty;
            let (nx, ny) = match d {
                Direction::East => {
                    let nx = map[y].iter().enumerate().find_map(|(i, c)| (c != &Empty).then_some(i)).unwrap();
                    (nx, y)
                },
                Direction::South => {
                    let ny = map.iter().enumerate().find_map(|(i, row)| (row.get(x)? != &Empty).then_some(i)).unwrap();
                    (x, ny)
                },
                Direction::West => {
                    let nx = map[y].iter().enumerate().rev().find_map(|(i, c)| (c != &Empty).then_some(i)).unwrap();
                    (nx, y)
                },
                Direction::North => {
                    let ny = map.iter().enumerate().rev().find_map(|(i, row)| (row.get(x)? != &Empty).then_some(i)).unwrap();
                    (x, ny)
                }
            };
            if map[ny][nx] == Cell::Floor {
                ((nx, ny), d)
            } else {
                ((x, y), d)
            }
        } else if map[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == Cell::Floor {
            (((x as i32 + dx) as usize, (y as i32 + dy) as usize), d)
        } else {
            ((x, y), d)
        }
    }
}

type Face = (Position, [(usize, Direction); 4]);
fn cube_neighbor<'a>(map: &'a Map, faces: &'a [Face; 6], size: Coord) -> impl Fn(Position, Direction) -> (Position, Direction) + 'a {
    use Direction::*;
    let find_face = move |(x, y): Position| faces.iter().find(|((fx, fy), _)| x >= *fx && y >= *fy && x - *fx < size && y - *fy < size).unwrap();
    move |(x, y): Position, d: Direction| {
        let ((dx, dy), dd) = {
            let ((face_x, face_y), neighbors) = find_face((x, y));
            let (fx, fy) = (x - *face_x, y - *face_y);
            let should_wrap = match d {
                    East => fx >= size - 1,
                    South => fy >= size - 1,
                    West => fx == 0,
                    North => fy == 0,
                };
            if should_wrap {
                let (dest_index, dest_direction) = neighbors[d as usize];
                let ((dest_face_x, dest_face_y), _) = faces[dest_index];
                let mirror = |x| size - 1 - x;
                let (dfx, dfy) = match (d, dest_direction) {
                    (East, East) | (South, South) | (West, West) | (North, North) => match d {
                        East => (0, fy),
                        South => (fx, 0),
                        West => (size - 1, fy),
                        North => (fx, size - 1),
                    },
                    (East, West) | (West, East) => (fx, mirror(fy)),
                    (North, South) | (South, North) => (mirror(fx), fy),
                    (East, North) | (West, South) => (fy, fx),
                    (East, South) | (North, West) | (South, East) | (West, North) => (mirror(fy), mirror(fx)),
                    (South, West) | (North, East) => (fy, fx),
                };
                ((dest_face_x + dfx, dest_face_y + dfy), dest_direction)
            } else {
                match d {
                    East => ((x + 1, y), d),
                    South => ((x, y + 1), d),
                    West => ((x - 1, y), d),
                    North => ((x, y - 1), d),
                }
            }
        };
        match map.get(dy).map(|row| row.get(dx)).flatten() {
            Some(Cell::Floor) => ((dx, dy), dd),
            _ => ((x, y), d)
        }
    }
}

fn graph_from_map(map: &Map, neighbor: impl Fn(Position, Direction) -> (Position, Direction)) -> Graph {
    use Direction::*;

    let make_floor = |x, y| Node::Floor([East, South, West, North].map(|d| neighbor((x, y), d)));

    map.iter()
       .enumerate()
       .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
       .filter_map(|(x, y, c)| match c {
           Cell::Empty => None,
           Cell::Wall => Some(((x, y), Node::Wall)),
           Cell::Floor => Some(((x, y), make_floor(x, y)))
       }).collect()
}

fn part1((map, instructions): &Data) -> usize {
    let neighbor = wrap_neighbor(map);
    let graph = graph_from_map(map, neighbor);
    let start_x = map[0].iter().enumerate().find_map(|(i, c)| (*c == Cell::Floor).then_some(i)).expect("No start position");
    let mut character = Character { position: (start_x, 0), heading: Direction::East };
    for instruction in instructions {
        match instruction {
            Instruction::Advance(x) => (0..*x).for_each(|_| character.advance(&graph)),
            Instruction::Left => character.turn_left(),
            Instruction::Right => character.turn_right(),
        }
    }
    let (x, y) = character.position;
    (y + 1) * 1000 + (x + 1) * 4 + character.heading as Coord
}

fn part2((map, instructions): &Data, faces: &[Face; 6], size: Coord) -> usize {
    // Collect moves for visualization
    let mut moves = HashMap::new();
    let neighbor = cube_neighbor(&map, faces, size);
    let graph = graph_from_map(map, neighbor);
    let start_x = map[0].iter().enumerate().find_map(|(i, c)| (*c == Cell::Floor).then_some(i)).expect("No start position");
    let mut character = Character { position: (start_x, 0), heading: Direction::East };
    for instruction in instructions {
        match instruction {
            Instruction::Advance(x) => (0..*x).for_each(|_| {
                moves.insert(character.position, character.heading);
                character.advance(&graph);
            }),
            Instruction::Left => character.turn_left(),
            Instruction::Right => character.turn_right(),
        }
    }
    moves.insert(character.position, character.heading);

    // Visualize path
    map.iter()
       .enumerate()
       .for_each(|(y, row)| {
           let line: String = row.iter()
               .enumerate()
               .map(|(x, c)| {
                   match moves.get(&(x, y)) {
                       Some(Direction::East) => '>',
                       Some(Direction::South) => 'v',
                       Some(Direction::West) => '<',
                       Some(Direction::North) => '^',
                       None => match c {
                           Cell::Empty => ' ',
                           Cell::Wall => '#',
                           Cell::Floor => '.'
                       }
                   }
               })
           .collect();
           println!("{line}");
       });

    let (x, y) = character.position;
    (y + 1) * 1000 + (x + 1) * 4 + character.heading as Coord
}

fn main() {
    let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data, &PUZZLE_FACES, PUZZLE_SIZE));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 6032);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 75388);
    }
    #[test]
    fn test_part2() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        let faces = [
            ((8, 0), [
             (5, Direction::West),
             (3, Direction::South),
             (2, Direction::South),
             (1, Direction::South),
            ]),
            ((0, 4), [
             (2, Direction::East),
             (4, Direction::North),
             (5, Direction::North),
             (0, Direction::South),
            ]),
            ((4, 4), [
             (3, Direction::East),
             (4, Direction::East),
             (1, Direction::West),
             (0, Direction::East),
            ]),
            ((8, 4), [
             (5, Direction::South),
             (4, Direction::South),
             (2, Direction::West),
             (0, Direction::North),
            ]),
            ((8, 8), [
             (5, Direction::East),
             (1, Direction::North),
             (2, Direction::North),
             (3, Direction::North),
            ]),
            ((12, 8), [
             (0, Direction::East),
             (1, Direction::East),
             (4, Direction::West),
             (3, Direction::West),
            ]),
        ];
        let size = 4;
        assert_eq!(part2(&data, &faces, size), 5031);
    }
    #[test]
    fn test_part2_custom() {
        let (_, data) = parse(include_str!("../custom_test_input.txt")).finish().expect("Parse error");
        let faces = [
            ((4, 0), [
             (1, Direction::East),
             (2, Direction::South),
             (4, Direction::East),
             (5, Direction::East),
            ]),
            ((8, 0), [
             (3, Direction::West),
             (2, Direction::West),
             (0, Direction::West),
             (5, Direction::North),
            ]),
            ((4, 4), [
             (1, Direction::North),
             (3, Direction::South),
             (4, Direction::South),
             (0, Direction::North),
            ]),
            ((4, 8), [
             (1, Direction::West),
             (5, Direction::West),
             (4, Direction::West),
             (2, Direction::North),
            ]),
            ((0, 8), [
             (3, Direction::East),
             (5, Direction::South),
             (0, Direction::East),
             (2, Direction::East),
            ]),
            ((0, 12), [
             (3, Direction::North),
             (1, Direction::South),
             (0, Direction::South),
             (4, Direction::North),
            ]),
            ];
        let size = 4;
        assert_eq!(part2(&data, &faces, size), 10008);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data, &PUZZLE_FACES, PUZZLE_SIZE), 182170);
    }
}
