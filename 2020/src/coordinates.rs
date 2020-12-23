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

pub fn manhattan_distance(lhs: Coord, rhs: Coord) -> isize {
    (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CoordN(Vec<isize>);

impl CoordN {
    pub fn origin(n: usize) -> CoordN {
        CoordN(vec![0; n])
    }

    pub fn from_vec(v: Vec<isize>) -> CoordN {
        CoordN(v)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeltaN(Vec<isize>);

impl DeltaN {
    pub fn from_vec(v: Vec<isize>) -> DeltaN {
        DeltaN(v)
    }
}

impl<'a, 'b> Add<&'b DeltaN> for &'a CoordN {
    type Output = CoordN;

    fn add(self, rhs: &'b DeltaN) -> Self::Output {
        let n = self.0.len();
        assert_eq!(n, rhs.0.len());
        let res: Vec<_> = (0..n).map(|i| self.0[i] + rhs.0[i]).collect();
        CoordN(res)
    }
}

impl<'a> AddAssign<&'a DeltaN> for CoordN {
    fn add_assign(&mut self, rhs: &'a DeltaN) {
        *self = &*self + rhs;
    }
}

impl<'a, 'b> Add<&'b DeltaN> for &'a DeltaN {
    type Output = DeltaN;

    fn add(self, rhs: &'b DeltaN) -> Self::Output {
        let n = self.0.len();
        assert_eq!(n, rhs.0.len());
        let res: Vec<_> = (0..n).map(|i| self.0[i] + rhs.0[i]).collect();
        DeltaN(res)
    }
}

impl<'a> AddAssign<&'a DeltaN> for DeltaN {
    fn add_assign(&mut self, rhs: &'a DeltaN) {
        *self = &*self + rhs;
    }
}
