use crate::coordinates::Coord;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Write};
use std::iter::FusedIterator;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Cw0,
    Cw90,
    Cw180,
    Cw270,
}

impl Rotation {
    pub fn to_cw_count(&self) -> usize {
        match *self {
            Rotation::Cw0 => 0,
            Rotation::Cw90 => 1,
            Rotation::Cw180 => 2,
            Rotation::Cw270 => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridIter<'a, T: 'a> {
    iter: std::slice::Iter<'a, T>,
    width: usize,
    coord: Coord,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            iter: grid.grid.iter(),
            width: grid.width,
            coord: Coord::origin(),
        }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.coord, self.iter.next()?);
        if self.coord.0 as usize == self.width - 1 {
            self.coord.0 = 0;
            self.coord.1 += 1;
        } else {
            self.coord.0 += 1;
        }
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> FusedIterator for GridIter<'a, T> {}

#[derive(Debug)]
pub struct GridIterMut<'a, T: 'a> {
    iter: std::slice::IterMut<'a, T>,
    width: usize,
    coord: Coord,
}

impl<'a, T> GridIterMut<'a, T> {
    pub fn new(grid: &'a mut Grid<T>) -> Self {
        Self {
            iter: grid.grid.iter_mut(),
            width: grid.width,
            coord: Coord::origin(),
        }
    }
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (Coord, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.coord, self.iter.next()?);
        if self.coord.0 as usize == self.width - 1 {
            self.coord.0 = 0;
            self.coord.1 += 1;
        } else {
            self.coord.0 += 1;
        }
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> FusedIterator for GridIterMut<'a, T> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::new();
        grid.resize_with(width * height, Default::default);
        Self { grid, width }
    }
}

impl<T> Grid<T> {
    pub fn from_vec_and_width(grid: Vec<T>, width: usize) -> Self {
        assert_eq!(grid.len() % width, 0);
        Self { grid, width }
    }

    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        GridIter::new(self)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        GridIterMut::new(self)
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.grid.iter_mut()
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

    pub fn flip_inplace(&mut self, axis: Axis) {
        let w = self.width() as isize;
        let h = self.height() as isize;
        match axis {
            Axis::Horizontal => {
                for y in 0..h / 2 {
                    for x in 0..w {
                        let i1 = self.index_for(Coord(x, y)).unwrap();
                        let i2 = self.index_for(Coord(x, h - 1 - y)).unwrap();
                        self.grid.swap(i1, i2);
                    }
                }
            }
            Axis::Vertical => {
                for x in 0..w / 2 {
                    for y in 0..h {
                        let i1 = self.index_for(Coord(x, y)).unwrap();
                        let i2 = self.index_for(Coord(w - 1 - x, y)).unwrap();
                        self.grid.swap(i1, i2);
                    }
                }
            }
        }
    }
}

impl<T> Grid<T>
where
    T: std::cmp::PartialEq,
{
    pub fn count_eq(&self, item: &T) -> usize {
        self.values().filter(|&e| e == item).count()
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn rotate_clockwise_inplace(&mut self) {
        let w = self.width() as isize;
        let h = self.height() as isize;
        let mut v = Vec::with_capacity(self.grid.len());

        for x in 0..w {
            for y in (0..h).rev() {
                v.push(self.get(Coord(x, y)).unwrap().clone());
            }
        }

        self.grid = v;
        self.width = h as usize;
    }

    pub fn rotate_inplace(&mut self, rotation: Rotation) {
        for _ in 0..rotation.to_cw_count() {
            self.rotate_clockwise_inplace();
        }
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

impl<T> Display for Grid<T>
where
    T: Clone,
    char: From<T>,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.height() as isize {
            for x in 0..self.width() as isize {
                let e = self.get(Coord(x, y)).unwrap();
                fmt.write_char(char::from(e.clone()))?;
            }
            fmt.write_char('\n')?;
        }
        Ok(())
    }
}
