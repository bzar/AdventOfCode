use itertools::Itertools;
type Stacks = Vec<Vec<char>>;
type Action = (usize, usize, usize);

fn parse_stacks(input: &str) -> Stacks {
    let lines: Vec<_> = input.lines().rev().collect();
    let stack_count = (lines.iter().map(|l| l.chars().count()).max().unwrap() + 1) / 4;
    let mut stacks: Vec<_> = (0..stack_count).map(|_| Vec::new()).collect();

    lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, ch)| ch.is_alphabetic())
        })
        .for_each(|(i, ch)| stacks[i].push(ch));
    stacks
}

fn parse_actions(input: &str) -> Vec<Action> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ').skip(1).step_by(2).map(|x| x.parse().unwrap()))
        .map(|mut xs| xs.next_tuple().unwrap())
        .map(|(n, from, to)| (n, from - 1, to - 1))
        .collect()
}

fn part1(stacks: &mut Stacks, (n, from, to): Action) {
    for _ in 0..n {
        let ch = stacks[from].pop().unwrap();
        stacks[to].push(ch);
    }
}

fn part2(stacks: &mut Stacks, (n, from, to): Action) {
    let crates: Vec<char> = (0..n).map(|_| stacks[from].pop().unwrap()).rev().collect();
    stacks[to].extend(crates.into_iter().rev());
}

fn execute(mut stacks: Stacks, actions: &Vec<Action>, command: fn(&mut Stacks, Action)) -> String {
    actions.iter().for_each(|i| command(&mut stacks, *i));
    let result = stacks.iter().map(|s| s.last()).collect::<Option<String>>();
    result.unwrap()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let (stacks_input, actions_input) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(&stacks_input);
    let actions = parse_actions(actions_input);

    println!("Part 1: {}", execute(stacks.clone(), &actions, part1));
    println!("Part 2: {}", execute(stacks, &actions, part2));
}
