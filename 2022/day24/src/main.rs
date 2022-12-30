use std::{collections::{BTreeMap, HashMap}, ops::Add};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
type Coord = usize;
type Position = (Coord, Coord);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North, South, East, West
}
struct Storm {
    direction: Direction,
    start: Position
}
       
struct Map {
    start: Position,
    goal: Position,
    area: Position,
    storms: Vec<Storm>
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Floor,
    Storm(Direction)
}

type Data = Vec<Vec<Cell>>;

impl Storm {
    fn predict(&self, t: u32, area: Position) -> Position {
        use Direction::*;
        let span = match self.direction {
            North | South => area.1 - 2,
            East | West => area.0 - 2
        } as i32;
        let start = match self.direction {
            North | South => self.start.1 - 1,
            East | West => self.start.0 - 1
        };
        let pos = match self.direction {
            South | East => (start + t as Coord) % span as Coord,
            North | West => ((start as i32 - (t as i32 % span) as i32 + span) % span) as Coord
        };

        match self.direction {
            North | South => (self.start.0, pos + 1),
            East | West => (pos + 1, self.start.1)
        }
    }
}

fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data> {
    use Cell::*;
    use Direction::*;
    let line = many1(alt((
        value(Wall, ncc::char('#')),
        value(Floor, ncc::char('.')),
        value(Storm(North), ncc::char('^')),
        value(Storm(South), ncc::char('v')),
        value(Storm(East), ncc::char('>')),
        value(Storm(West), ncc::char('<')),
    )));
    let data = separated_list1(ncc::line_ending, line);
    all_consuming(delimited(ncc::multispace0, data, ncc::multispace0))(input)
}

fn map_from_data(data: &Data) -> Map {
    let storms: Vec<_> = data.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .filter_map(|(x, y, c)| if let Cell::Storm(direction) = c {
            Some(Storm { direction, start: (x, y) })
        } else {
            None
        })
    .collect();

    let start = (data.first().unwrap().iter().positions(|c| c == &Cell::Floor).next().unwrap(), 0);
    let goal = (data.last().unwrap().iter().positions(|c| c == &Cell::Floor).next().unwrap(), data.len() - 1);
    let area = (data.iter().map(|row| row.len()).max().unwrap(), data.len());
    Map { start, goal, area, storms }
}
fn astar<Id: std::hash::Hash + Eq, Node: Clone, Distance: Ord + Default + Add<Output=Distance> + Copy>(start: Node, node_id: impl Fn(&Node) -> Id, neighbors: impl Fn(&Node) -> Vec<(Distance, Node)>, is_goal: impl Fn(&Node) -> bool) -> Option<Vec<Id>> {
    let mut queue: BTreeMap<Distance, Vec<Node>> = [(Distance::default(), vec![start.clone()])].into();
    let mut parents: HashMap<Id, Node> = HashMap::new();

    while let Some((_, ref node)) = queue.iter_mut().find_map(|(p, xs)| xs.pop().map(|x| (p, x))) {
        if is_goal(&node) {
            return (0..)
                .scan(Some(node.clone()), move |pos, _| {
                    if pos.is_none() {
                        return None;
                    }
                    let node = pos.clone().unwrap();
                    let id = node_id(&node);

                    if node_id(&node) == node_id(&start) {
                        *pos = None;
                        Some(id)
                    } else {
                        *pos = Some(parents.get(&id)?.clone());
                        Some(id)
                    }
                })
                .collect::<Vec<_>>()
                    .into()
        }
        for (d, ref n) in neighbors(&node) {
            let nid = node_id(n);
            parents.entry(nid).or_insert_with(|| {
                queue.entry(d).or_default().push(n.clone());
                node.clone()
            });
        }
    }
    None
}
fn path_length(map: &Map, path: &[(Coord, Coord)]) -> usize {
    let is_wall = |(x, y)| (x, y) != map.start
        && (x, y) != map.goal
        && (x == 0 || y == 0 || x >= map.area.0 - 1 || y >= map.area.1 - 1); 
    let node_id = |n: &(u32, Coord, Coord)| *n;
    let goal_distance = |(x, y): (Coord, Coord)| x.abs_diff(map.goal.0) + y.abs_diff(map.goal.1);
    let neighbors = |(t, x, y): &(u32, Coord, Coord)| {
        [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
            .map(|(dx, dy)| (t + 1, (*x as i32 + dx) as Coord, (*y as i32 + dy) as Coord))
            .into_iter()
            .filter(|(_, x, y)| !is_wall((*x, *y)))
            .filter(|(t, x, y)| map.storms.iter().all(|s| s.predict(*t, map.area) != (*x, *y)))
            .map(|(t, x, y)| (goal_distance((x, y)) + t as Coord, (t, x, y)))
            .collect()
    };
    let mut t = 0;
    for i in 0..path.len() - 1 {
        let start = path[i];
        let goal = path[i+1];
        let is_goal = |(_, x, y): &(_, Coord, Coord)| (*x, *y) == goal;
        let path = astar((t as u32, start.0, start.1), node_id, neighbors, is_goal);
        t += path.unwrap().len() - 1 ;
    }
    t
}
fn part1(data: &Data) -> usize {
    let map = map_from_data(data);
    path_length(&map, &[map.start, map.goal])
}

fn part2(data: &Data) -> usize {
    let map = map_from_data(data);
    path_length(&map, &[map.start, map.goal, map.start, map.goal])
}

fn main() {
    let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 18);
    }
    #[test]
    fn test_custom() {
        let input = ["###.##",
                     "#...<#",
                     "#.v..#",
                     "#.####"].join("\n");
    let (_, data) = parse(&input).finish().expect("Parse error");
    assert_eq!(part1(&data), 7);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 225);
    }
    #[test]
    fn test_part2() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 54);
    }
    #[test]
    fn test_part2_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 711);
    }
}
