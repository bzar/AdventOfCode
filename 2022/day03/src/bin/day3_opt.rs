use itertools::Itertools;
use std::iter::Peekable;

fn priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => u32::from(ch) - u32::from('a') + 1,
        'A'..='Z' => u32::from(ch) - u32::from('A') + 27,
        _ => unreachable!(),
    }
}
fn find_first_common_from_sorted<'a, const N: usize>(
    iterators: &'a mut [Peekable<impl Iterator<Item = &'a u32>>; N],
) -> u32 {
    loop {
        if iterators
            .iter_mut()
            .map(|it| it.peek().unwrap())
            .all_equal()
        {
            return **iterators[0].peek().unwrap();
        }
        let (it, _) = iterators
            .iter_mut()
            .map(|it| {
                let v = *it.peek().unwrap();
                (it, v)
            })
            .min_by_key(|x| x.1)
            .unwrap();
        it.next();
    }
}
fn main() {
    let mut data = Vec::with_capacity(10240);
    std::io::read_to_string(std::io::stdin())
        .expect("Error reading data")
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.chars().map(priority).chain([u32::MAX].into_iter()))
        .for_each(|x| data.push(x));

    data.as_mut_slice()
        .split_mut(|x| *x == u32::MAX)
        .for_each(|xs| {
            let (left, right) = xs.split_at_mut(xs.len() / 2);
            left.sort_unstable();
            right.sort_unstable();
        });

    let part1: u32 = data
        .split(|x| *x == u32::MAX)
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            find_first_common_from_sorted(&mut [a.iter().peekable(), b.iter().peekable()])
        })
        .sum();

    data.as_mut_slice()
        .split_mut(|x| *x == u32::MAX)
        .for_each(|xs| xs.sort_unstable());

    let part2: u32 = data
        .split(|x| *x == u32::MAX)
        .tuples()
        .map(|(a, b, c)| {
            find_first_common_from_sorted(&mut [
                a.iter().peekable(),
                b.iter().peekable(),
                c.iter().peekable(),
            ])
        })
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
