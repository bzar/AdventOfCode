use rayon::prelude::*;
use nom::{
    character::complete as ncc,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{delimited, terminated},
    combinator::{opt, all_consuming},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

type Amount = u32;
#[derive(Debug, Default, Copy, Clone)]
struct Resources {
    ore: Amount,
    clay: Amount,
    obsidian: Amount,
    geode: Amount
}

impl std::ops::Add<Resources> for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode
        }
    }
}
impl Resources {
    fn covers(&self, r: &Resources) -> bool {
       self.ore >= r.ore
           && self.clay >= r.clay
           && self.obsidian >= r.obsidian
           && self.geode >= r.geode 
    }
    fn consume(&self, r: &Resources) -> Option<Resources> {
        self.covers(r).then_some(Resources {
            ore: self.ore - r.ore,
            clay: self.clay - r.clay,
            obsidian: self.obsidian - r.obsidian,
            geode: self.geode - r.geode
        })
    }
}
type Id = u32;
#[derive(Debug)]
struct Blueprint {
    id: Id,
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources
}
fn parse(input: &str) -> nom::IResult<&str, Vec<Blueprint>> {
    fn cost(i: &str) -> nom::IResult<&str, Resources> {
        let (rest, ore) = opt(terminated(ncc::u32, tag(" ore")))(i)?;
        let (rest, _) = opt(tag(" and "))(rest)?;
        let (rest, clay) = opt(terminated(ncc::u32, tag(" clay")))(rest)?;
        let (rest, _) = opt(tag(" and "))(rest)?;
        let (rest, obsidian) = opt(terminated(ncc::u32, tag(" obsidian")))(rest)?;
        Ok((rest, Resources { ore: ore.unwrap_or(0), clay: clay.unwrap_or(0), obsidian: obsidian.unwrap_or(0), geode: 0 }))
    }
    fn blueprint(input: &str) -> nom::IResult<&str, Blueprint> {
        let (rest, id) = delimited(tag("Blueprint "), ncc::u32, tag(": "))(input)?;
        let (rest, ore) = delimited(tag("Each ore robot costs "), cost, tag(". "))(rest)?;
        let (rest, clay) = delimited(tag("Each clay robot costs "), cost, tag(". "))(rest)?;
        let (rest, obsidian) = delimited(tag("Each obsidian robot costs "), cost, tag(". "))(rest)?;
        let (rest, geode) = delimited(tag("Each geode robot costs "), cost, tag("."))(rest)?;

        Ok((rest, Blueprint { id, ore, clay, obsidian, geode }))
    }
    all_consuming(delimited(ncc::multispace0, separated_list1(ncc::line_ending, blueprint), ncc::multispace0))(input)
}

fn blueprint_geodes(blueprint: &Blueprint, time: u32) -> u32 {
    println!("{blueprint:?}");
    let resources = Resources::default();
    let mut robots = Resources::default();
    robots.ore = 1;

    let best = std::sync::atomic::AtomicU32::new(0);
    let (sender, receiver) = crossbeam::channel::unbounded();
    sender.send((0, resources, robots)).unwrap();

    receiver.iter().par_bridge().for_each(|(t, resources, robots)| {
        let best_local = best.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |val| Some(val.max(resources.geode))).unwrap();
        let time_left = time - t;
        if time_left == 0 || resources.geode + (robots.geode + time_left) * time_left < best_local {
            return;
        }

        sender.send((t + 1, resources + robots, robots)).unwrap();
        if let Some(resources) = resources.consume(&blueprint.ore) {
            let mut rs = robots;
            rs.ore += 1;
            sender.send((t + 1, resources + robots, rs)).unwrap();
        }
        if let Some(resources) = resources.consume(&blueprint.clay) {
            let mut rs = robots;
            rs.clay += 1;
            sender.send((t + 1, resources + robots, rs)).unwrap();
        }
        if let Some(resources) = resources.consume(&blueprint.obsidian) {
            let mut rs = robots;
            rs.obsidian += 1;
            sender.send((t + 1, resources + robots, rs)).unwrap();
        }
        if let Some(resources) = resources.consume(&blueprint.geode) {
            let mut rs = robots;
            rs.geode += 1;
            sender.send((t + 1, resources + robots, rs)).unwrap();
        }
    });

    best.load(std::sync::atomic::Ordering::SeqCst)
}
fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.par_iter().map(|bp| bp.id * blueprint_geodes(bp, 24)).sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.par_iter().take(3).map(|bp| blueprint_geodes(bp, 32)).product()
}

fn main() {
    let (_, blueprints) = parse(PUZZLE_INPUT).finish().expect("Parse error");
    //println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn test_part1() {
        let (_, blueprint) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&blueprint), 33);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, blueprints) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&blueprints), 1356);
    }
    /*
    #[test]
    fn test_part2() {
        let (_, moves) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&moves), 1_514_285_714_288);
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, sensors) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&sensors, 0, 4_000_000), Some(10649103160102));
    }
    */
}




