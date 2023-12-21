use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish,
};

const PUZZLE_INPUT: &str = include_str!("../data/day19.txt");

#[derive(Debug, Clone, Copy)]
enum Rule<'a> {
    Less(&'a str, u64, &'a str),
    Greater(&'a str, u64, &'a str),
    Otherwise(&'a str),
}
type Workflow<'a> = (&'a str, Vec<Rule<'a>>);
type Part<'a> = Vec<(&'a str, u64)>;
type Model<'a> = (Vec<Workflow<'a>>, Vec<Part<'a>>);

fn parse(input: &str) -> nom::IResult<&str, Model> {
    let parse_less = map(
        tuple((
            ncc::alpha1,
            preceded(tag("<"), ncc::u64),
            preceded(tag(":"), ncc::alpha1),
        )),
        |(name, value, destination)| Rule::Less(name, value, destination),
    );
    let parse_greater = map(
        tuple((
            ncc::alpha1,
            preceded(tag(">"), ncc::u64),
            preceded(tag(":"), ncc::alpha1),
        )),
        |(name, value, destination)| Rule::Greater(name, value, destination),
    );
    let parse_otherwise = map(ncc::alpha1, Rule::Otherwise);
    let parse_rule = alt((parse_less, parse_greater, parse_otherwise));
    let parse_workflow = tuple((
        take_until("{"),
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}")),
    ));
    let parse_part = delimited(
        tag("{"),
        separated_list1(tag(","), separated_pair(ncc::alpha1, tag("="), ncc::u64)),
        tag("}"),
    );
    all_consuming(separated_pair(
        many1(terminated(parse_workflow, ncc::line_ending)),
        ncc::line_ending,
        many1(terminated(parse_part, ncc::line_ending)),
    ))(input)
}

fn eval_rule<'a>(rule: &Rule<'a>, part: &HashMap<&str, u64>) -> Option<&'a str> {
    match rule {
        Rule::Less(name, value, result) => (part.get(name)? < value).then_some(result),
        Rule::Greater(name, value, result) => (part.get(name)? > value).then_some(result),
        Rule::Otherwise(result) => Some(result),
    }
}
fn eval_workflow<'a>(rules: &Vec<Rule<'a>>, part: &HashMap<&str, u64>) -> Option<&'a str> {
    rules.iter().find_map(|r| eval_rule(r, part))
}
fn sort_part<'a>(workflows: &HashMap<&str, Vec<Rule<'a>>>, part: &HashMap<&str, u64>) -> &'a str {
    let mut name = "in";
    while name != "A" && name != "R" {
        let workflow = workflows.get(name).expect("Workflow not found");
        name = eval_workflow(workflow, part).expect("Workflow eval failed");
    }
    name
}
fn part1(input: &str) -> u64 {
    let (_, model) = parse(input).finish().unwrap();
    let (workflow_specs, part_specs) = model;
    let workflows: HashMap<_, _> = workflow_specs.into_iter().collect();
    let parts: Vec<HashMap<_, _>> = part_specs
        .into_iter()
        .map(|part| part.into_iter().collect())
        .collect();

    parts
        .into_iter()
        .filter(|p| sort_part(&workflows, p) == "A")
        .map(|p| p.values().sum::<u64>())
        .sum()
}
fn part2(input: &str) -> u64 {
    let (_, model) = parse(input).finish().unwrap();
    let (workflow_specs, _) = model;
    let workflows: HashMap<_, _> = workflow_specs.into_iter().collect();

    let mut stack = vec![("in", [1, 4001], [1, 4001], [1, 4001], [1, 4001])];
    let mut total: u64 = 0;

    while let Some((name, mut x, mut m, mut a, mut s)) = stack.pop() {
        if name == "A" {
            total += (x[1] - x[0]) * (m[1] - m[0]) * (a[1] - a[0]) * (s[1] - s[0]);
        } else if name != "R" {
            let workflow = workflows.get(name).expect("Unknown workflow");
            for rule in workflow {
                match rule {
                    Rule::Less(name, v, result) => {
                        match *name {
                            "x" => {
                                stack.push((result, [x[0], *v], m, a, s));
                                x[0] = *v;
                            }
                            "m" => {
                                stack.push((result, x, [m[0], *v], a, s));
                                m[0] = *v;
                            }
                            "a" => {
                                stack.push((result, x, m, [a[0], *v], s));
                                a[0] = *v;
                            }
                            "s" => {
                                stack.push((result, x, m, a, [s[0], *v]));
                                s[0] = *v;
                            }
                            _ => unreachable!(),
                        };
                    }
                    Rule::Greater(name, v, result) => {
                        let v = *v + 1;
                        match *name {
                            "x" => {
                                stack.push((result, [v, x[1]], m, a, s));
                                x[1] = v;
                            }
                            "m" => {
                                stack.push((result, x, [v, m[1]], a, s));
                                m[1] = v;
                            }
                            "a" => {
                                stack.push((result, x, m, [v, a[1]], s));
                                a[1] = v;
                            }
                            "s" => {
                                stack.push((result, x, m, a, [v, s[1]]));
                                s[1] = v;
                            }
                            _ => unreachable!(),
                        };
                    }
                    Rule::Otherwise(result) => stack.push((result, x, m, a, s)),
                }
            }
        }
    }
    total
}

fn main() {
    println!("Part 1: {}", part1(PUZZLE_INPUT));
    println!("Part 2: {}", part2(PUZZLE_INPUT));
}

#[cfg(test)]
mod test_day19 {
    use super::*;
    const TEST_INPUT: &str = include_str!("../data/day19_test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 19114);
    }
    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(PUZZLE_INPUT), 409898);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 167409079868000);
    }
    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(PUZZLE_INPUT), 113057405770956);
    }
}
