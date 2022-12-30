use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as ncc,
    combinator::{map, value},
    multi::{many1, many_m_n},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
type Stacks = Vec<Vec<char>>;
type Action = (usize, usize, usize);

fn parse_stacks(input: &str) -> IResult<&str, Stacks> {
    let filled_slot = map(
        delimited(ncc::char('['), ncc::anychar, ncc::char(']')),
        Some,
    );
    let empty_slot = value(None, tag("   "));
    let slot = terminated(alt((filled_slot, empty_slot)), many_m_n(0, 1, tag(" ")));
    let parse_line = terminated(many1(slot), ncc::newline);
    let (rest, values) = many1(parse_line)(input)?;

    let stacks = values
        .into_iter()
        .rev()
        .flat_map(|row| row.into_iter().enumerate())
        .fold(Stacks::new(), |mut stacks, (s, ch)| {
            if let Some(ch) = ch {
                if s >= stacks.len() {
                    stacks.resize(s + 1, Vec::new());
                }
                stacks.get_mut(s).unwrap().push(ch);
            }
            stacks
        });
    Ok((rest, stacks))
}

fn parse_actions(input: &str) -> IResult<&str, Vec<Action>> {
    let parse_action = terminated(
        map(
            tuple((
                preceded(tag("move "), ncc::u32),
                preceded(tag(" from "), ncc::u32),
                preceded(tag(" to "), ncc::u32),
            )),
            |(a, b, c)| (a as usize, b as usize, c as usize),
        ),
        ncc::newline,
    );

    many1(parse_action)(input)
}

fn part1(stacks: &mut Stacks, (n, from, to): Action) {
    for _ in 0..n {
        let ch = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(ch);
    }
}

fn part2(stacks: &mut Stacks, (n, from, to): Action) {
    let crates: Vec<char> = (0..n)
        .map(|_| stacks[from - 1].pop().unwrap())
        .rev()
        .collect();
    stacks[to - 1].extend(crates.into_iter().rev());
}

fn execute(mut stacks: Stacks, actions: &Vec<Action>, command: fn(&mut Stacks, Action)) -> String {
    actions.iter().for_each(|i| command(&mut stacks, *i));
    let result = stacks.iter().map(|s| s.last()).collect::<Option<String>>();
    result.unwrap()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let (stacks_input, actions_input) = input.split_once("\n\n").unwrap();
    let (_, stacks) = parse_stacks(&stacks_input).expect("Error parsing stacks");
    let (_, actions) = parse_actions(actions_input).expect("Error parsing actions");

    println!("Part 1: {}", execute(stacks.clone(), &actions, part1));
    println!("Part 2: {}", execute(stacks, &actions, part2));
}
