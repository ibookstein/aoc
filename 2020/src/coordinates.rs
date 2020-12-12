use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::{From, TryFrom};
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Turn {
    Front = 0,
    Right = 1,
    Back = 2,
    Left = 3,
}

impl Mul<Turn> for isize {
    type Output = Turn;

    fn mul(self, rhs: Turn) -> Self::Output {
        let n = self.rem_euclid(4) as u8;
        Turn::try_from((n * u8::from(rhs)) % 4).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn turn(self, to: Turn) -> Self {
        let res = (u8::from(self) + u8::from(to)) % 4;
        Direction::try_from(res).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(pub isize, pub isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Delta(pub isize, pub isize);

impl Coord {
    pub fn origin() -> Coord {
        Coord(0, 0)
    }
}

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

impl From<Direction> for Delta {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Delta(0, -1),
            Direction::Down => Delta(0, 1),
            Direction::Left => Delta(-1, 0),
            Direction::Right => Delta(1, 0),
        }
    }
}

pub fn manhattan_distance(lhs: Coord, rhs: Coord) -> isize {
    (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()
}
