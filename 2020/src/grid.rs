use crate::coordinates::Coord;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

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
