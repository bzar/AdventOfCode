use std::collections::{HashMap, VecDeque};

use nom::{
    IResult, Finish,
    branch::alt,
    character::complete as ncc,
    combinator::{map, value},
    multi::{separated_list0, many0}
};

#[derive(Copy, Clone, Debug)]
enum Node {
    Start(u32),
    Path(u32),
    End(u32)
}
type Map = Vec<Vec<Node>>;
type Position = (usize, usize);

impl Node {
    fn height(&self) -> u32 {
        match self {
            Self::Start(h) => *h,
            Self::Path(h) => *h,
            Self::End(h) => *h
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Node>>> {
    let node = alt((
            map(ncc::satisfy(|ch| ('a'..='z').contains(&ch)), |ch| Node::Path(ch as u32 - 'a' as u32)),
            value(Node::Start('a' as u32 - 'a' as u32), ncc::char('S')),
            value(Node::End('z' as u32 - 'a' as u32), ncc::char('E'))));
    separated_list0(ncc::line_ending, many0(node))(input)
}

fn nodes<'a>(map: &'a Map) -> impl Iterator<Item=(&'a Node, Position)> {
    map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, n)| (n, (x, y))))
}

fn find_start_and_end(map: &Map) -> Option<(Position, Position)> {
    let (start, end) = nodes(map)
        .fold((None, None), |(start, end), (n, pos)| {
            match n {
                Node::Start(_) => (Some(pos), end),
                Node::End(_) => (start, Some(pos)),
                Node::Path(_) => (start, end)
            }
        });

    if let (Some(start), Some(end)) = (start, end) {
        Some((start, end))
    } else {
        None
    }
}

fn neighbors<'a>(map: &'a Map, (x, y): Position) -> impl Iterator<Item=Position> + 'a {
    let height = map[y][x].height();
    let (x, y) = (x as i32, y as i32);
    [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
        .into_iter()
        .filter_map(|(x, y)| Some((usize::try_from(x).ok()?, usize::try_from(y).ok()?)))
        .filter_map(|(x, y)| Some((map.get(y)?.get(x)?, (x, y))))
        .filter_map(move |(n, pos)| (n.height() <= height + 1).then_some(pos)) 
}

fn shortest_path<'a>(map: &Map, starts: &'a [Position], end: Position) -> Option<impl Iterator<Item=Position> + 'a> {
    let num_nodes: usize = map.iter().map(Vec::len).sum::<usize>();
    let mut queue = VecDeque::with_capacity(num_nodes/10);
    queue.extend(starts.iter().copied());
    let mut parents = HashMap::with_capacity(num_nodes/4);

    while let Some(node) = queue.pop_front() {
        if node == end {
            return (0..).scan(node, move |pos, _| {
                let node = *pos;
                *pos = *parents.get(pos)?;
                (!starts.contains(&node)).then_some(*pos)
            })
            .into()
        }
        for n in neighbors(map, node) {
            parents.entry(n).or_insert_with(|| {
                queue.push_back(n);
                node
            });
        }
    }
    None
}
fn part1(input: &str) -> usize {
    let (_, map) = parse(input).finish().expect("Parse error");
    let (start, end) = find_start_and_end(&map).expect("No start or end");
    shortest_path(&map, &[start], end).expect("No path").count()
}

fn part2(input: &str) -> usize {
    let (_, map) = parse(input).finish().expect("Parse error");
    let (_, end) = find_start_and_end(&map).expect("No start or end");
    let starts: Vec<_> = nodes(&map)
        .filter_map(|(n, pos)| (n.height() == 0).then_some(pos))
        .collect();
    shortest_path(&map, &starts, end).expect("No path").count()
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../test_input.txt")), 31);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../test_input.txt")), 29);
    }
}

