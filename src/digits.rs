use num_traits::{sign::Unsigned, AsPrimitive, One, Zero};
use std::ops::{DivAssign, MulAssign, RemAssign};

pub trait UnsignedDigits<T>:
    Unsigned
    + Copy
    + One
    + Zero
    + PartialOrd
    + MulAssign<T>
    + RemAssign<T>
    + DivAssign<T>
    + AsPrimitive<usize>
{
}

impl<
        T: Unsigned
            + Copy
            + One
            + Zero
            + PartialOrd
            + MulAssign<T>
            + RemAssign<T>
            + DivAssign<T>
            + AsPrimitive<usize>,
    > UnsignedDigits<T> for T
{
}

pub fn digits<T>(mut num: T, radix: T) -> impl Iterator<Item = T>
where
    T: UnsignedDigits<T>,
{
    let zero = T::zero();

    let mut divisor = T::one();
    while num >= divisor * radix {
        divisor *= radix;
    }

    std::iter::from_fn(move || {
        if divisor == zero {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= radix;
            Some(v)
        }
    })
}
