use nom::{character::complete as ncc, sequence as ns};

fn range_parser(i: &str) -> nom::IResult<&str, (i32, i32)> {
    ns::separated_pair(ncc::i32, ncc::char('-'), ncc::i32)(i)
}

fn part1(input: &str) -> i32 {
    let contains = |(a0, b0): (i32, i32), (a1, b1): (i32, i32)| a0 <= a1 && b0 >= b1;
    nom::multi::fold_many0(
        ns::terminated(
            ns::separated_pair(range_parser, ncc::char(','), range_parser),
            ncc::multispace0,
        ),
        || 0,
        move |result, (r1, r2)| {
            (contains(r1, r2) || contains(r2, r1))
                .then_some(result + 1)
                .unwrap_or(result)
        },
    )(input)
    .unwrap()
    .1
}

fn part2(input: &str) -> i32 {
    let overlaps = |(a0, b0): (i32, i32), (a1, b1): (i32, i32)| a0 <= b1 && a1 <= b0;
    nom::multi::fold_many0(
        ns::terminated(
            ns::separated_pair(range_parser, ncc::char(','), range_parser),
            ncc::multispace0,
        ),
        || 0,
        move |result, (r1, r2)| overlaps(r1, r2).then_some(result + 1).unwrap_or(result),
    )(input)
    .unwrap()
    .1
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
