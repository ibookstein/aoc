use aoc::{
    aoc_input::get_input,
    coordinates::{Coord, Delta},
};

#[derive(Debug, Clone, Copy)]
struct TargetArea {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

fn iter_two_items<T>(mut iter: impl Iterator<Item = T>) -> Result<(T, T), &'static str> {
    let first = iter.next().ok_or("Iter first error")?;
    let second = iter.next().ok_or("Iter second error")?;
    if iter.next().is_some() {
        return Err("Too many items");
    }
    Ok((first, second))
}

fn parse_prefix_and_split_two<'a>(
    s: &'a str,
    prefix: &str,
    split_pat: &str,
) -> Result<(&'a str, &'a str), &'static str> {
    if !s.starts_with(prefix) {
        return Err("Invalid prefix");
    }

    iter_two_items(s[prefix.len()..].split(split_pat))
}

fn parse_target_area(input: &str) -> Result<TargetArea, &'static str> {
    let (x_str, y_str) = parse_prefix_and_split_two(input, "target area: ", ", ")?;
    let (xmin_str, xmax_str) = parse_prefix_and_split_two(x_str, "x=", "..")?;
    let (ymin_str, ymax_str) = parse_prefix_and_split_two(y_str, "y=", "..")?;

    let xmin: isize = xmin_str.parse().or(Err("Failed parsing xmin"))?;
    let xmax: isize = xmax_str.parse().or(Err("Failed parsing xmax"))?;
    let ymin: isize = ymin_str.parse().or(Err("Failed parsing ymin"))?;
    let ymax: isize = ymax_str.parse().or(Err("Failed parsing ymax"))?;

    if xmin > xmax || ymin > ymax {
        return Err("Invalid ranges");
    }
    if xmin <= 0 || ymax >= 0 {
        return Err("Target area is not completely within fourth quadrant");
    }

    Ok(TargetArea {
        xmin,
        xmax,
        ymin,
        ymax,
    })
}

fn simulate(area: TargetArea, initial_v: Delta) -> bool {
    let mut cur = Coord::origin();
    let mut v = initial_v;
    let xrange = area.xmin..=area.xmax;
    let yrange = area.ymin..=area.ymax;

    loop {
        cur += v;
        let Coord(x, y) = cur;

        if xrange.contains(&x) && yrange.contains(&y) {
            return true;
        }

        if x > area.xmax || y < area.ymin {
            return false;
        }

        v.0 -= v.0.signum();
        v.1 -= 1;
    }
}

fn successful_initial_velocities(area: TargetArea) -> usize {
    let mut total = 0usize;
    for v_y in area.ymin..-area.ymin {
        for v_x in 1..=area.xmax {
            total += simulate(area, Delta(v_x, v_y)) as usize;
        }
    }
    total
}

fn main() {
    let input = get_input(2021, 17);
    let area = parse_target_area(input.trim()).unwrap();

    // Denote the initial velocity (v_x, v_y).
    // The series of the probe's displacements have y coordinates:
    // y_0 = 0
    // y_1 = v_y
    // y_2 = v_y + (v_y - 1) = 2 * v_y - 1
    // y_3 = v_y + (v_y - 1) + (v_y - 2) = 3 * v_y - 3
    // The general formula is:
    // y_n = n*v_y - (n^2 - n) / 2 = -0.5n^2 + (v_y + 0.5)n
    // We need to maximize v_y while still having at least one i for which
    // ymin <= y_i <= ymax (for large enough values of v_y, the fall will
    // overshoot the target area rather than intersect it).
    // This is a parabola, so the maximum is achieved at
    // n = -(v_y + 0.5) / 2*(-0.5) = v_y + 0.5. Because v_y and n are integers,
    // and because a parabola is symmetric with respect to its extremum, the
    // maximum for our purposes will be at n = v_y, with a value of:
    // -0.5v_y^2 + (v_y + 0.5)v_y = 0.5(v_y^2 + v_y)
    // The delta between two consecutive y-displacements (n, n + 1) is v_y - n.
    // Second intersection with 0 is at n = 2*v_y + 1, after which the probe
    // immediately plummets to v_y - (2*v_y + 1) = -v_y - 1.
    // Therefore, assuming ymin < 0, for v_y = -ymin we immediately overshoot
    // the target area after intersecting y = 0.

    let max_height_v_y = -area.ymin - 1;
    let max_height = max_height_v_y * (max_height_v_y + 1) / 2;
    println!("Style shot maximum height: {}", max_height);

    let total = successful_initial_velocities(area);
    println!("Successful initial velocities: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const EXAMPLE_AREA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_simulate() {
        let area = parse_target_area(EXAMPLE_AREA).unwrap();
        assert!(simulate(area, Delta(6, 8)));
        assert!(simulate(area, Delta(23, -10)));
        assert!(simulate(area, Delta(8, 0)));
        assert!(simulate(area, Delta(25, -7)));
        assert!(!simulate(area, Delta(17, -4)));
    }

    #[test]
    fn test_successful_initial_velocities() {
        let area = parse_target_area(EXAMPLE_AREA).unwrap();
        let total = successful_initial_velocities(area);
        assert_eq!(total, 112);
    }
}
