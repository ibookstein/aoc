use num_traits::{Signed, Zero};
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VecN<T, const N: usize>([T; N]);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Scalar<T>(T);

impl<T: Copy + Zero, const N: usize> VecN<T, N> {
    pub fn zero() -> Self {
        Self([T::zero(); N])
    }

    pub fn new(inner: [T; N]) -> Self {
        Self(inner)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, const N: usize> Index<usize> for VecN<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy + SubAssign, const N: usize> SubAssign<VecN<T, N>> for VecN<T, N> {
    fn sub_assign(&mut self, rhs: VecN<T, N>) {
        for i in 0..self.0.len() {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<T: Copy + SubAssign, const N: usize> Sub<VecN<T, N>> for VecN<T, N> {
    type Output = VecN<T, N>;

    fn sub(self, rhs: VecN<T, N>) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<T: Copy + AddAssign, const N: usize> AddAssign<VecN<T, N>> for VecN<T, N> {
    fn add_assign(&mut self, rhs: VecN<T, N>) {
        for i in 0..self.0.len() {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<T: Copy + AddAssign, const N: usize> Add<VecN<T, N>> for VecN<T, N> {
    type Output = VecN<T, N>;

    fn add(self, rhs: VecN<T, N>) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<T: Copy + MulAssign<T>, const N: usize> Mul<VecN<T, N>> for Scalar<T> {
    type Output = VecN<T, N>;

    fn mul(self, rhs: VecN<T, N>) -> Self::Output {
        let mut res = rhs;
        for c in res.0.iter_mut() {
            *c *= self.0;
        }
        res
    }
}

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for VecN<T, N> {
    type Output = VecN<T, N>;

    fn neg(self) -> Self::Output {
        let mut res = self;
        for c in res.0.iter_mut() {
            *c = -*c;
        }
        res
    }
}

impl<T: Zero + Copy + Signed + AddAssign, const N: usize> VecN<T, N> {
    pub fn manhattan_norm(&self) -> T {
        let mut norm = T::zero();
        for c in self.0.iter() {
            norm += c.abs();
        }
        norm
    }
}

pub type ISizeVec3 = VecN<isize, 3>;
