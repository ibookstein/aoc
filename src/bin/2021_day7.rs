use aoc::aoc_input::get_input;

fn fuel2_single(position: isize, x: isize) -> isize {
    let delta = (position - x).abs();
    delta * (delta + 1) / 2
}

fn fuel2_total(positions: &[isize], x: isize) -> isize {
    positions.iter().map(|&p| fuel2_single(p, x)).sum()
}

fn main() {
    let input = get_input(2021, 7);
    let mut positions: Vec<isize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    positions.sort();

    let median = positions[positions.len() / 2];
    let fuel_const_rate: isize = positions.iter().map(|p| (*p - median).abs()).sum();
    println!("Fuel required (constant rate): {}", fuel_const_rate);

    /*
    let sum: isize = positions.iter().sum();
    // Not rounded to nearest, but yields the correct result when checked
    // against brute-forcing. It's unclear to me whether this is always correct,
    // since the function is f(x) = Sigma_i(|x - p_i| * (|x - p_i| + 1) / 2)
    // and the derivative would have an ugly piecewise component (the average
    // zeroes the other non-piecewise component)...
    let average = sum / positions.len() as isize;
    let fuel_var_rate = fuel2_total(&positions, average);
    */

    let min = positions[0];
    let max = positions[positions.len() - 1];
    let fuel_var_rate = (min..max)
        .map(|p| fuel2_total(&positions, p))
        .min()
        .unwrap();
    println!("Fuel required (variable rate): {}", fuel_var_rate);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_fuel2_single() {
        assert_eq!(fuel2_single(0, 5), 15);
        assert_eq!(fuel2_single(16, 5), 66);
    }

    #[test]
    fn test_fuel2_total() {
        let positions = [16isize, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(fuel2_total(&positions, 5), 168);
        assert_eq!(fuel2_total(&positions, 2), 206);
    }
}
