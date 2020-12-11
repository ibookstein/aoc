use std::convert::{TryFrom, TryInto};
use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(pub isize, pub isize);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Delta(pub isize, pub isize);

impl Add<Delta> for Coord {
    type Output = Coord;

    fn add(self, rhs: Delta) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Delta> for Coord {
    fn add_assign(&mut self, rhs: Delta) {
        *self = *self + rhs;
    }
}

impl Mul<Delta> for isize {
    type Output = Delta;

    fn mul(self, rhs: Delta) -> Self::Output {
        Delta(self * rhs.0, self * rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    fn index_for(&self, c: Coord) -> Option<usize> {
        let Coord(x, y) = c;
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        let (w, h) = (self.width(), self.height());
        if x < w && y < h {
            Some(x + w * y)
        } else {
            None
        }
    }

    pub fn get(&self, c: Coord) -> Option<&T> {
        let idx = self.index_for(c)?;
        self.grid.get(idx)
    }

    pub fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        let idx = self.index_for(c)?;
        self.grid.get_mut(idx)
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<T> = Vec::new();
        let mut width: Option<usize> = None;

        for line in s.lines() {
            if line.is_empty() {
                return Err("Empty line");
            }

            let mut cur_width = 0usize;
            for c in line.chars() {
                let item: T = c.try_into().or(Err("Failed parsing char"))?;
                grid.push(item);
                cur_width += 1;
            }

            if width.is_some() && width.unwrap() != cur_width {
                return Err("Non-uniform line length");
            }

            width = Some(cur_width);
        }

        let width = width.ok_or("No lines")?;
        Ok(Grid { grid, width })
    }
}
