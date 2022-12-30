use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete as ncc,
    multi::separated_list1,
    sequence::{preceded, tuple},
    branch::alt,
    Finish,
};
use std::collections::{HashMap, HashSet, VecDeque};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
type Valve<'a> = (&'a str, u32, Vec<&'a str>);
type Valves<'a> = Vec<Valve<'a>>;
type Location = usize;
type Map = HashMap<Location, HashSet<Location>>;
type Flow = u32;

fn parse(input: &str) -> nom::IResult<&str, Valves> {
    let line = tuple((
        preceded(tag("Valve "), ncc::alpha1),
        preceded(tag(" has flow rate="), ncc::u32),
        preceded(
            alt((tag("; tunnels lead to valves "), 
                tag("; tunnel leads to valve "))),
            separated_list1(tag(", "), ncc::alpha1),
        ),
    ));
    separated_list1(ncc::line_ending, line)(input)
}

fn distance(start: Location, end: Location, map: &Map) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut stack: VecDeque<_> = [(start, 0)].into();
    while let Some((pos, steps)) = stack.pop_front() {
        visited.insert(pos);
        if pos == end {
            return Some(steps);
        } else {
            let neigbors = map.get(&pos).unwrap().iter().filter(|n| !visited.contains(n)).map(|n| (*n, steps + 1));
            stack.extend(neigbors);
        }
    }
    None
}

fn release_pressure<const N: usize>(valves: &Valves, duration: u32) -> u32 {
    let name_id_map: HashMap<&str, usize> = valves
        .iter()
        .flat_map(|v| v.2.iter())
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect();
    let get_id = move |name| *name_id_map.get(name).unwrap();
    let start = get_id("AA");
    let mut map = Map::new();
    for (name, _, adjacent_names) in valves.iter() {
        let location = get_id(name);
        let adjacent: Vec<_> = adjacent_names.iter().map(|n| get_id(*n)).collect();
        map.entry(location).or_default().extend(adjacent.iter());
        for a in adjacent {
            map.entry(a).or_default().insert(location);
        }
    }
    let costs: HashMap<(Location, Location), u32> = map.keys().flat_map(|l| {
        map.keys()
         .map(|x| ((*l, *x), distance(*l, *x, &map).unwrap()))
    }).collect();
    let actionable: HashSet<_> = valves
        .iter()
        .filter_map(|(name, flow, _)| (*flow > 0).then_some((get_id(name), *flow)))
        .collect();
    let mut best = 0; //release_pressure(&costs, actionable.iter(), start);
    type Plan = Vec<(Location, Flow)>;
    let mut queue: VecDeque<[Plan; N]> = [[0; N].map(|_| vec![(start, 0u32)])].into();
    while let Some(plans) = queue.pop_back() {
            let mut left = actionable.clone();
            let mut released: u32 = 0;
            let mut time_left = 0;
            let mut next = 0;
            for (i, plan) in plans.iter().enumerate() {
                let mut time = duration;
                let mut it = plan.iter();
                let (first, _) = *it.next().unwrap();
                released += it.scan(first, |pos, &(next, flow)| {
                    left.remove(&(next, flow));
                    time = time.saturating_sub(*costs.get(&(*pos, next)).unwrap() + 1);
                    *pos = next;
                    Some(flow * time)
                })
                .sum::<u32>();
                if time > time_left {
                    time_left = time;
                    next = i;
                }
            }
            if left.is_empty() {
                if released > best {
                    best = released;
                }
            } else {
                // Estimate ability to release pressure by assuming opening the most pressured
                // valve each turn
                let sorted_left: Vec<_> = left.iter().map(|(_, flow)| *flow).sorted().collect();
                let times = (0..=time_left).rev();
                let estimate_left: u32 = sorted_left.iter().rev().zip(times).map(|(flow, t)| flow * t).sum::<u32>() * N as u32;
                let potential: u32 = released + estimate_left;
                if potential > best {
                    left.into_iter().for_each(|x| {
                        let mut new_plan = plans.clone();
                        new_plan[next].push(x);
                        queue.push_back(new_plan);
                    })
                }
            }
        
    }
    return best;
}
fn part1(valves: &Valves) -> u32 {
    release_pressure::<1>(valves, 30)
}

fn part2(valves: &Valves) -> u32 {
    release_pressure::<2>(valves, 26)
}

fn main() {
    let (_, valves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    println!("Part 1: {}", part1(&valves));
    println!("Part 2: {}", part2(&valves));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, valves) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&valves), 1651);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, valves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&valves), 1673);
    }
    #[test]
    fn test_part2() {
        let (_, valves) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&valves), 1707);
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, valves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&valves), 2343);
    }
}
