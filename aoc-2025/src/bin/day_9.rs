use chumsky::prelude::*;

const SAMPLE_DATA: &str = include_str!("day_9_sample.txt");
const PUZZLE_DATA: &str = include_str!("day_9_puzzle.txt");

type Coord = u64;
type Coords = (Coord, Coord);
type Input = Vec<Coords>;
type Output = u64;

#[derive(Debug, Clone)]
enum Line {
    Horizontal((Coord, Coord), Coord),
    Vertical(Coord, (Coord, Coord)),
}

impl Line {
    fn new((x0, y0): Coords, (x1, y1): Coords) -> Self {
        if x0 == x1 {
            Line::Vertical(x0, (y0.min(y1), y0.max(y1)))
        } else if y0 == y1 {
            Line::Horizontal((x0.min(x1), x0.max(x1)), y0)
        } else {
            panic!("Diagonal")
        }
    }
    fn contains(&self, (x, y): &Coords) -> bool {
        match self {
            Line::Horizontal((x0, x1), y0) => y == y0 && x0 <= x && x <= x1,
            Line::Vertical(x0, (y0, y1)) => x == x0 && y0 <= y && y <= y1,
        }
    }
    fn intersects(&self, other: &Line) -> bool {
        let result = match (self, other) {
            (Line::Horizontal((x0, x1), y0), Line::Horizontal((x2, x3), y1)) => {
                y0 == y1 && ((x0..=x1).contains(&x2) || (x2..=x3).contains(&x0))
            }
            (Line::Horizontal(_, y), Line::Vertical(x, _)) => {
                self.contains(&(*x, *y)) && other.contains(&(*x, *y))
            }
            (Line::Vertical(x, _), Line::Horizontal(_, y)) => {
                self.contains(&(*x, *y)) && other.contains(&(*x, *y))
            }
            (Line::Vertical(x0, (y0, y1)), Line::Vertical(x1, (y2, y3))) => {
                x0 == x1 && ((y0..=y1).contains(&y2) || (y2..=y3).contains(&y0))
            }
        };
        result
    }
    fn intersects_horizontal_line_at(&self, y: &Coord) -> bool {
        match self {
            Line::Horizontal(_, y0) => y == y0,
            Line::Vertical(_, (y0, y1)) => y0 <= y && y <= y1,
        }
    }

    fn is_left_of(&self, x: &Coord) -> bool {
        match self {
            Line::Horizontal((x0, _), _) => x < x0,
            Line::Vertical(x0, _) => x < x0,
        }
    }
}
#[derive(Debug)]
struct Border {
    lines: Vec<Line>,
}
impl Border {
    fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }
    fn contains(&self, (x, y): &Coords) -> bool {
        if self.lines.iter().any(|l| l.contains(&(*x, *y))) {
            return true;
        }

        self.lines
            .iter()
            .filter(|l| l.intersects_horizontal_line_at(y) && l.is_left_of(x))
            .count()
            % 2
            == 1
    }
    fn intersects(&self, line: &Line) -> bool {
        self.lines.iter().any(|l| line.intersects(l))
    }
}
fn parser<'src>() -> impl Parser<'src, &'src str, Input> {
    let coord = text::int(10).map(|s: &str| s.parse::<Coord>().unwrap());
    let coords = coord.then_ignore(just(',')).then(coord);
    coords.separated_by(text::newline()).collect().padded()
}
fn square_size((x0, y0): &Coords, (x1, y1): &Coords) -> Coord {
    (x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1)
}
fn part_1(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| input.iter().skip(i + 1).map(move |t2| (t1, t2)))
        .map(|(a, b)| square_size(a, b))
        .max()
        .unwrap()
}

fn define_border(input: &Input) -> Border {
    let pairs = input.iter().zip(input.iter().cycle().skip(1));
    let lines = pairs.map(|(a, b)| Line::new(*a, *b)).collect();
    Border::new(lines)
}

fn square_inner_lines(&(x0, y0): &Coords, &(x1, y1): &Coords) -> [Line; 4] {
    let (minx, miny, maxx, maxy) = (x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1));
    let (x0, y0, x1, y1) = (
        (minx + 1).min(maxx),
        (miny + 1).min(maxy),
        (maxx - 1).max(minx),
        (maxy - 1).max(miny),
    );
    let top = Line::new((x0, y0), (x1, y0));
    let bottom = Line::new((x0, y1), (x1, y1));
    let left = Line::new((x0, y0), (x0, y1));
    let right = Line::new((x1, y0), (x1, y1));
    [top, bottom, left, right]
}
fn part_2(input: &Input) -> Output {
    let border = define_border(input);

    input
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| input.iter().skip(i + 1).map(move |t2| (t1, t2)))
        .filter(|((x0, y0), (x1, y1))| border.contains(&(x0.min(&x1) + 1, y0.min(&y1) + 1)))
        .filter(|(a, b)| {
            square_inner_lines(a, b)
                .iter()
                .all(|c| !border.intersects(&c))
        })
        .map(|(a, b)| square_size(a, b))
        .max()
        .unwrap()
}

fn main() {
    let input = parser().parse(PUZZLE_DATA).unwrap();
    let part_1_result = part_1(&input);
    println!("part 1: {part_1_result}");
    let part_2_result = part_2(&input);
    println!("part 2: {part_2_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_1(&input), 50);
    }

    #[test]
    fn part_2_sample() {
        let input = parser().parse(SAMPLE_DATA).unwrap();
        assert_eq!(part_2(&input), 24);
    }
}
