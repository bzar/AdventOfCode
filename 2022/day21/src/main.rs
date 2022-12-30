use std::collections::HashMap;
use nom::{
    character::complete as ncc,
    multi::separated_list1,
    sequence::{delimited, terminated, separated_pair, tuple},
    combinator::{all_consuming, map},
    branch::alt,
    Finish, bytes::complete::tag,
};

const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Node<'a> {
    Value(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Human
}
type Data<'a> = Vec<(&'a str, Node<'a>)>;
fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Data<'a>> {
    let name = terminated(ncc::alpha1, tag(": "));
    let node = alt((
            map(ncc::i64, Node::Value),
            map(separated_pair(ncc::alpha1, tag(" + "), ncc::alpha1), |(a, b)| Node::Add(a, b)),
            map(separated_pair(ncc::alpha1, tag(" - "), ncc::alpha1), |(a, b)| Node::Sub(a, b)),
            map(separated_pair(ncc::alpha1, tag(" * "), ncc::alpha1), |(a, b)| Node::Mul(a, b)),
            map(separated_pair(ncc::alpha1, tag(" / "), ncc::alpha1), |(a, b)| Node::Div(a, b))
            ));
    let line = tuple((name, node));
    let data = separated_list1(ncc::line_ending, line);
    all_consuming(delimited(ncc::multispace0, data, ncc::multispace0))(input)
}

fn eval<'a>(mut nodes: HashMap<&'a str, Node<'a>>, name: &'a str, human: Option<i64>) -> (Option<i64>, HashMap<&'a str, Node<'a>>) {
    let mut stack: Vec<_> = [name].into();
    use Node::*;
    while let Some(name) = stack.pop() {
        let node = *nodes.get(name).expect("Node not found");
        match node {
            Value(_) => continue,
            Add(a, b) => {
                if let (Some(Value(a)), Some(Value(b))) = (nodes.get(a), nodes.get(b)) {
                    nodes.insert(name, Value(a + b));
                } else {
                    stack.push(name);
                    stack.push(a);
                    stack.push(b);
                }
            },
            Sub(a, b) => {
                if let (Some(Value(a)), Some(Value(b))) = (nodes.get(a), nodes.get(b)) {
                    nodes.insert(name, Value(a - b));
                } else {
                    stack.push(name);
                    stack.push(a);
                    stack.push(b);
                }
            },
            Mul(a, b) => {
                if let (Some(Value(a)), Some(Value(b))) = (nodes.get(a), nodes.get(b)) {
                    nodes.insert(name, Value(a * b));
                } else {
                    stack.push(name);
                    stack.push(a);
                    stack.push(b);
                }
            },
            Div(a, b) => {
                if let (Some(Value(a)), Some(Value(b))) = (nodes.get(a), nodes.get(b)) {
                    nodes.insert(name, Value(a / b));
                } else {
                    stack.push(name);
                    stack.push(a);
                    stack.push(b);
                }
            },
            Human => if let Some(value) = human {
                nodes.insert(name, Value(value));
            } else {
                return (None, nodes);
            }
        }
    }

    if let Some(Value(result)) = nodes.get(name) {
        return (Some(*result), nodes);
    } else {
        unreachable!()
    }
}

fn part1(data: &Data) -> i64 {
    let nodes: HashMap<_, _> = data.iter().copied().collect();
    eval(nodes, "root", None).0.unwrap()
}

fn determine_human<'a>(mut nodes: HashMap<&'a str, Node<'a>>, name: &'a str, result: i64) -> i64 {
    use Node::*;
    let node = *nodes.get(name).expect("Node not found");
    println!("{name} = {node:?} should equal {result}");
    let (value, human) = match node {
        Value(_) => unreachable!(),
        Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
            let va;
            let vb;
            (va, nodes) = eval(nodes, a, None);
            (vb, nodes) = eval(nodes, b, None);
            if let Some(value) = va {
                println!("  {a} = {value}");
                (value, b)
            } else if let Some(value) = vb {
                println!("  {b} = {value}");
                (value, a)
            } else {
                unreachable!()
            }
        },
        Human => return result
    };
    match node {
        Add(_, _) => determine_human(nodes, human, result - value),
        Sub(a, _) if a == human => determine_human(nodes, human, result + value),
        Sub(_, b) if b == human => determine_human(nodes, human, value - result),
        Mul(_, _) => determine_human(nodes, human, result / value),
        Div(a, _) if a == human => determine_human(nodes, human, value * result),
        Div(_, b) if b == human => determine_human(nodes, human, value / result),
        _ => unreachable!()
    }
}
fn part2(data: &Data) -> i64 {
    let mut nodes: HashMap<_, _> = data.iter().copied().collect();
    nodes.insert("humn", Node::Human);
    let root = *nodes.get("root").unwrap();
    let (a, b) = match root {
        Node::Add(a, b) | Node::Sub(a, b) | Node::Mul(a, b) | Node::Div(a, b)  => (a, b),
        _ => unreachable!()
    };
    let va;
    let vb;
    (va, nodes) = eval(nodes, a, None);
    (vb, nodes) = eval(nodes, b, None);
    let (value, node) = {
        if let Some(value) = va {
            (value, b)
        } else if let Some(value) = vb {
            (value, a)
        } else {
            unreachable!()
        }
    };

    determine_human(nodes, node, value)
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
        assert_eq!(part1(&data), 152);
    }
    #[test]
    fn test_part1_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part1(&data), 291425799367130);
    }
    #[test]
    fn test_part2() {
        let (_, data) = parse(TEST_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 301);
    }

    #[test]
    fn test_part2_puzzle() {
        let (_, data) = parse(PUZZLE_INPUT).finish().expect("Parse error");
        assert_eq!(part2(&data), 3219579395609);
    }
}
