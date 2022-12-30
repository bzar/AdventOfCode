use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::many0,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
enum LsItem<'a> {
    File((u32, &'a str)),
    Dir(&'a str),
}
#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<LsItem<'a>>),
}

#[derive(Default, Debug)]
struct State<'a> {
    pwd: PathBuf,
    files: HashMap<PathBuf, Vec<(u32, &'a str)>>,
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    let parse_cd = terminated(preceded(tag("$ cd "), ncc::not_line_ending), ncc::line_ending);
    let parse_file = map(separated_pair(ncc::u32, ncc::space1, ncc::not_line_ending), LsItem::File);
    let parse_dir = map(preceded(tag("dir "), ncc::not_line_ending), LsItem::Dir);
    let parse_ls_line = terminated(alt((parse_file, parse_dir)), ncc::line_ending);
    let parse_ls = preceded(terminated(tag("$ ls"), ncc::line_ending), many0(parse_ls_line));
    all_consuming(many0(alt((map(parse_cd, Command::Cd), map(parse_ls, Command::Ls)))))(input)
}

fn execute<'s>(state: &mut State<'s>, command: &'s Command) {
    match command {
        Command::Cd("/") => state.pwd = "/".into(),
        Command::Cd("..") => { state.pwd.pop(); }
        Command::Cd(dir) => state.pwd.push(dir),
        Command::Ls(items) => state.files
            .entry(state.pwd.clone())
            .or_insert(Vec::new())
            .extend(items.iter().filter_map(|item| match item {
                LsItem::File(f) => Some(f),
                LsItem::Dir(_) => None,
            })),
    };
}

fn directory_size(state: &State, path: &PathBuf) -> u32 {
    state
        .files
        .iter()
        .filter(|(p, _)| p.starts_with(path))
        .flat_map(|(_, files)| files.iter().map(|(size, _)| size))
        .sum()
}

fn part1(state: &State) -> u32 {
    const MAX_SIZE: u32 = 100_000;
    state.files.keys()
        .map(|k| directory_size(&state, k))
        .filter(|size| *size <= MAX_SIZE)
        .sum()
}

fn part2(state: &State) -> Option<u32> {
    const TOTAL_SPACE: u32 = 70_000_000;
    const NEED_SPACE: u32 = 30_000_000;

    let used_space = directory_size(&state, &"/".into());
    let unused_space = TOTAL_SPACE - used_space;
    let to_delete = NEED_SPACE - unused_space;

    state.files.keys()
        .map(|k| directory_size(&state, k))
        .filter(|size| *size >= to_delete)
        .min()
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let (_, commands) = parse(&input).finish().expect("Error parsing commands");
    let state = commands.iter().fold(State::default(), |mut state, c| {
        execute(&mut state, c);
        state
    });

    println!("Part 1: {}", part1(&state));
    println!("Part 2: {}", part2(&state).expect("No suitable directory found"));
}
