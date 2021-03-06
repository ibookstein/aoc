use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::{From, TryFrom};
use std::ops::{Add, AddAssign, Mul};
use strum_macros::EnumIter;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, IntoPrimitive, TryFromPrimitive)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord(pub isize, pub isize);

impl Coord {
    pub fn origin() -> Coord {
        Coord(0, 0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Delta(pub isize, pub isize);

impl Delta {
    pub fn turn(&self, t: Turn) -> Delta {
        match t {
            Turn::Front => *self,
            Turn::Right => Delta(-self.1, self.0),
            Turn::Back => Delta(-self.0, -self.1),
            Turn::Left => Delta(self.1, -self.0),
        }
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

impl Add<Delta> for Delta {
    type Output = Delta;

    fn add(self, rhs: Delta) -> Self::Output {
        Delta(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Delta> for Delta {
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

impl From<Direction8> for Delta {
    fn from(d: Direction8) -> Self {
        match d {
            Direction8::North => Delta(0, -1),
            Direction8::NorthEast => Delta(1, -1),
            Direction8::East => Delta(1, 0),
            Direction8::SouthEast => Delta(1, 1),
            Direction8::South => Delta(0, 1),
            Direction8::SouthWest => Delta(-1, 1),
            Direction8::West => Delta(-1, 0),
            Direction8::NorthWest => Delta(-1, -1),
        }
    }
}

pub fn manhattan_distance(lhs: Coord, rhs: Coord) -> isize {
    (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CoordVec(Vec<isize>);

impl CoordVec {
    pub fn origin(n: usize) -> Self {
        Self(vec![0; n])
    }

    pub fn from_vec(v: Vec<isize>) -> Self {
        Self(v)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeltaVec(Vec<isize>);

impl DeltaVec {
    pub fn from_vec(v: Vec<isize>) -> Self {
        Self(v)
    }
}

impl<'a, 'b> Add<&'b DeltaVec> for &'a CoordVec {
    type Output = CoordVec;

    fn add(self, rhs: &'b DeltaVec) -> Self::Output {
        let n = self.0.len();
        assert_eq!(n, rhs.0.len());
        let res: Vec<_> = (0..n).map(|i| self.0[i] + rhs.0[i]).collect();
        CoordVec(res)
    }
}

impl<'a> AddAssign<&'a DeltaVec> for CoordVec {
    fn add_assign(&mut self, rhs: &'a DeltaVec) {
        *self = &*self + rhs;
    }
}

impl<'a, 'b> Add<&'b DeltaVec> for &'a DeltaVec {
    type Output = DeltaVec;

    fn add(self, rhs: &'b DeltaVec) -> Self::Output {
        let n = self.0.len();
        assert_eq!(n, rhs.0.len());
        let res: Vec<_> = (0..n).map(|i| self.0[i] + rhs.0[i]).collect();
        DeltaVec(res)
    }
}

impl<'a> AddAssign<&'a DeltaVec> for DeltaVec {
    fn add_assign(&mut self, rhs: &'a DeltaVec) {
        *self = &*self + rhs;
    }
}
