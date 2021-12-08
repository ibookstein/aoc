use num_integer::Integer;

pub fn div_round_nearest(dividend: isize, divisor: isize) -> isize {
    let sign = 1 - ((dividend < 0) ^ (divisor < 0)) as isize * 2;
    let (q, r) = dividend.div_rem(&divisor);
    q + (r.abs() > (divisor / 2).abs()) as isize * sign
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_div_round_nearest() {
        assert_eq!(div_round_nearest(8, 3), 3);
        assert_eq!(div_round_nearest(8, -3), -3);
        assert_eq!(div_round_nearest(-8, 3), -3);
        assert_eq!(div_round_nearest(-8, -3), 3);

        assert_eq!(div_round_nearest(10, 3), 3);
        assert_eq!(div_round_nearest(10, -3), -3);
        assert_eq!(div_round_nearest(-10, 3), -3);
        assert_eq!(div_round_nearest(-10, -3), 3);
    }
}
