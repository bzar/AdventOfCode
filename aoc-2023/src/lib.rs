use std::{
    collections::{BTreeMap, HashMap},
    ops::Add,
};

pub type Coord = i64;
pub type Coords = [Coord; 2];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectMap<T: Clone + core::fmt::Debug>(Vec<Vec<T>>);

impl<T: Clone + core::fmt::Debug> RectMap<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        let width = rows.first().map(|r| r.len()).unwrap_or(0);
        assert!(rows.iter().skip(1).all(|r| r.len() == width));
        Self(rows)
    }
    pub fn new_from_size(width: usize, height: usize, value: &T) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| value.clone()).collect())
            .collect();
        Self(map)
    }
    pub fn width(&self) -> Coord {
        self.0.first().map(|r| r.len()).unwrap_or(0) as Coord
    }
    pub fn height(&self) -> Coord {
        self.0.len() as Coord
    }
    pub fn rows<'a>(
        &'a self,
    ) -> impl Iterator<Item = impl Iterator<Item = &'a T> + Clone + core::fmt::Debug>
           + 'a
           + DoubleEndedIterator
           + ExactSizeIterator
           + core::fmt::Debug {
        self.0.iter().map(|row| row.iter())
    }
    pub fn columns<'a>(
        &'a self,
    ) -> impl Iterator<Item = impl Iterator<Item = &'a T> + Clone + core::fmt::Debug>
           + 'a
           + DoubleEndedIterator
           + ExactSizeIterator
           + core::fmt::Debug {
        // Assume each row has equal length
        (0..self.0[0].len()).map(|x| self.0.iter().map(move |row| &row[x]))
    }
    pub fn cells<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Coords, &'a T)> + DoubleEndedIterator + core::fmt::Debug {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, value)| ([x as Coord, y as Coord], value))
        })
    }

    pub fn get(&self, [x, y]: Coords) -> Option<&T> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.0.get(y)?.get(x)
    }
    pub fn get_mut(&mut self, [x, y]: Coords) -> Option<&mut T> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.0.get_mut(y)?.get_mut(x)
    }
    pub fn set(&mut self, &[x, y]: &Coords, value: T) -> bool {
        let inner = || {
            let x: usize = x.try_into().ok()?;
            let y: usize = y.try_into().ok()?;
            *self.0.get_mut(y)?.get_mut(x)? = value;
            Some(())
        };
        inner().is_some()
    }
    pub fn modify(&mut self, coords: Coords, f: impl Fn(&T) -> T) -> Option<&T> {
        let value = self.get_mut(coords)?;
        *value = f(value);
        Some(value)
    }
    pub fn adjacent<'a>(
        &'a self,
        pos: &'a Coords,
    ) -> impl Iterator<Item = (Direction, Coords)> + 'a {
        Direction::all()
            .into_iter()
            .filter_map(|dir| Some((dir, dir.apply(*pos)?)))
            .filter(|(_, pos)| self.get(*pos).is_some())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    pub fn apply(&self, coords: Coords) -> Option<Coords> {
        self.apply_n(coords, 1)
    }
    pub fn apply_n(&self, [x, y]: Coords, n: Coord) -> Option<Coords> {
        match self {
            Direction::East => Some([x.checked_add(n)?, y]),
            Direction::North => Some([x, y.checked_sub(n)?]),
            Direction::West => Some([x.checked_sub(n)?, y]),
            Direction::South => Some([x, y.checked_add(n)?]),
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
    pub fn all() -> [Direction; 4] {
        [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ]
    }
}
pub fn astar<
    Id: std::hash::Hash + Eq,
    Node: Clone,
    Distance: Ord + Default + Add<Output = Distance> + Copy,
>(
    start: Node,
    node_id: impl Fn(&Node) -> Id,
    neighbors: impl Fn(&Node) -> Vec<(Distance, Node)>,
    is_goal: impl Fn(&Node) -> bool,
) -> Option<Vec<Id>> {
    let mut queue: BTreeMap<Distance, Vec<Node>> =
        [(Distance::default(), vec![start.clone()])].into();
    let mut parents: HashMap<Id, Node> = HashMap::new();

    while let Some((&nd, ref node)) = queue
        .iter_mut()
        .find_map(|(p, xs)| xs.pop().map(|x| (p, x)))
    {
        if is_goal(&node) {
            return (0..)
                .scan(Some(node.clone()), move |pos, _| {
                    if pos.is_none() {
                        return None;
                    }
                    let node = pos.clone().unwrap();
                    let id = node_id(&node);

                    if node_id(&node) == node_id(&start) {
                        *pos = None;
                        Some(id)
                    } else {
                        *pos = Some(parents.get(&id)?.clone());
                        Some(id)
                    }
                })
                .collect::<Vec<_>>()
                .into();
        }
        for (d, ref n) in neighbors(&node) {
            let nid = node_id(n);
            parents.entry(nid).or_insert_with(|| {
                queue.entry(nd + d).or_default().push(n.clone());
                node.clone()
            });
        }
    }
    None
}
