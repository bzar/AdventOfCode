use itertools::Itertools;
use std::collections::HashSet;

static PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let lines: Vec<_> = std::io::stdin()
        .lines()
        .map(|l| l.expect("Error reading line"))
        .filter(|l| !l.is_empty())
        .collect();
    let part1: usize = lines
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            let set: HashSet<_> = a.chars().collect();
            let letter = b.chars().find(|c| set.contains(c)).unwrap();
            let priority = 1 + PRIORITY.find(letter).unwrap();
            priority
        })
        .sum();

    let part2: usize = lines
        .iter()
        .tuples()
        .map(|(a, b, c)| {
            let set_a: HashSet<_> = a.chars().collect();
            let set_b: HashSet<_> = b.chars().collect();
            let set_ab: HashSet<_> = set_a.intersection(&set_b).collect();
            let letter = c.chars().find(|ch| set_ab.contains(ch)).unwrap();
            let priority = 1 + PRIORITY.find(letter).unwrap();
            priority
        })
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
