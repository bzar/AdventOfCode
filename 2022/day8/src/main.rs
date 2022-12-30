use itertools::Itertools;
use bittle::{Bits, BitsMut};
type Trees = Vec<Vec<u8>>;

fn ray<'a>(
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    trees: &'a Trees,
) -> impl Iterator<Item = (usize, usize, &'a u8)> {
    (0..)
        .map(move |i| ((x + i * dx) as usize, (y + i * dy) as usize))
        .take_while(|(x, y)| *y < trees.len() && *x < trees[*y].len())
        .map(|(x, y)| (x, y, &trees[y][x]))
}
fn scenic_score((y, x): (usize, usize), trees: &Trees) -> usize {
    if y == 0 || x == 0 || y == trees.len() - 1 || x == trees[y].len() - 1 {
        0
    } else {
        [
            (x, y - 1, 0, -1),
            (x, y + 1, 0, 1),
            (x - 1, y, -1, 0),
            (x + 1, y, 1, 0),
        ]
        .map(|(xx, yy, dx, dy)| {
            ray(xx as i32, yy as i32, dx, dy, trees)
                .map(|(_, _, t)| *t)
                .fold((0, 0), |(tallest, seen), t| {
                    (tallest >= trees[y][x])
                        .then_some((tallest, seen))
                        .unwrap_or((t.max(tallest), seen + 1))
                })
                .1
        })
        .into_iter()
        .product()
    }
}
fn part1(trees: &Trees) -> u32 {
    let height = trees.len() as i32;
    let width = trees.iter().map(|row| row.len()).max().unwrap() as i32;
    let mut visible = vec![0u32; (width * height) as usize / 32 + 1];
    let views = [
        (0..1, 0..height, 1, 0),
        (width - 1..width, 0..height, -1, 0),
        (0..width, 0..1, 0, 1),
        (0..width, height - 1..height, 0, -1),
    ];
    for (xs, ys, dx, dy) in views {
        xs.cartesian_product(ys).for_each(|(x, y)| {
            visible.set_bit((x + y * width as i32) as u32);
            let mut r = ray(x, y, dx, dy, trees);
            let (_, _, mut h) = r.next().unwrap();
            r.for_each(|(x, y, t)| {
                if t > h {
                    visible.set_bit((x + y * width as usize) as u32);
                    h = t;
                }
            });
        });
    }
    visible.count_ones()
}

fn part2(trees: &Trees) -> usize {
    (0..trees.len())
        .cartesian_product(0..trees[0].len())
        .map(|pos| scenic_score(pos, trees))
        .max()
        .unwrap()
}

fn parse(input: &str) -> Option<Trees> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| ch.to_digit(10).map(|d| d as u8))
                .collect::<Option<Vec<_>>>()
        })
        .collect()
}
fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let trees = parse(&input).expect("parse error");
    println!("Part 1: {}", part1(&trees));
    println!("Part 2: {}", part2(&trees));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse("30373\n25512\n65332\n33549\n35390").unwrap()),
            21
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse("30373\n25512\n65332\n33549\n35390").unwrap()),
            8
        );
    }
}
