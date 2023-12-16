pub type Coord = usize;
pub type Coords = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectMap<T: Clone + core::fmt::Debug>(Vec<Vec<T>>);

impl<T: Clone + core::fmt::Debug> RectMap<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        let width = rows.first().map(|r| r.len()).unwrap_or(0);
        assert!(rows.iter().skip(1).all(|r| r.len() == width));
        Self(rows)
    }
    pub fn width(&self) -> usize {
        self.0.first().map(|r| r.len()).unwrap_or(0)
    }
    pub fn height(&self) -> usize {
        self.0.len()
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
    ) -> impl Iterator<Item = ((usize, usize), &'a T)> + DoubleEndedIterator + core::fmt::Debug
    {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, value)| ((x, y), value))
        })
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.0.get(y)?.get(x)
    }
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        self.0.get_mut(y)?.get_mut(x)
    }
    pub fn modify(&mut self, (x, y): (usize, usize), f: impl Fn(&T) -> T) -> Option<&T> {
        let value = self.0.get_mut(y)?.get_mut(x)?;
        *value = f(value);
        Some(value)
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
    pub fn apply(&self, (x, y): Coords) -> Option<Coords> {
        match self {
            Direction::East => Some((x.checked_add(1)?, y)),
            Direction::North => Some((x, y.checked_sub(1)?)),
            Direction::West => Some((x.checked_sub(1)?, y)),
            Direction::South => Some((x, y.checked_add(1)?)),
        }
    }
}
