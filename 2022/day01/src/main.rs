use itertools::Itertools;

fn main() {
    let results: Vec<i32> = std::io::stdin().read_to_string.expect("Error reading stdin")
        .lines()

        .batching(|it| {
            it.take_while(|s| !s.is_empty())
                .map(|s| i32::from_str_radix(&s, 10).expect("Error parsing value"))
                .sum1()
        })
        .sorted_unstable_by(|a, b| Ord::cmp(b, a))
        .collect();
    println!("Part 1: {}", results.first().unwrap_or(&0));
    println!("Part 2: {}", results.iter().take(3).sum::<i32>());
}
