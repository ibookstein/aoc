use aoc::{aoc_input::get_input, coordinates::Coord};
use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LineSegment {
    start: Coord,
    end: Coord,
}

impl FromStr for LineSegment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let start = split.next().ok_or("Segment separator missing")?;
        let end = split.next().ok_or("Segment end missing")?;
        if split.next().is_some() {
            return Err("Trailing garbage");
        }

        let parse_coord = |s: &str| {
            let mut split = s.split(',');
            let x = split.next().ok_or("Start x missing")?;
            let y = split.next().ok_or("Start y missing")?;
            if split.next().is_some() {
                return Err("Trailing garbage");
            }

            let x: isize = x.parse().or(Err("Failed parsing x"))?;
            let y: isize = y.parse().or(Err("Failed parsing y"))?;
            Ok(Coord(x, y))
        };

        let start = parse_coord(start)?;
        let end = parse_coord(end)?;

        Ok(Self { start, end })
    }
}

fn main() {
    let input = get_input(2021, 5);
    let segments: Vec<_> = input
        .trim()
        .lines()
        .map(|s| s.parse::<LineSegment>().unwrap())
        .collect();

    let mut diagram1 = HashMap::<Coord, usize>::new();
    let mut diagram2 = HashMap::<Coord, usize>::new();

    let mut insert = |c: Coord, only2: bool| {
        if !only2 {
            *diagram1.entry(c).or_insert(0) += 1;
        }
        *diagram2.entry(c).or_insert(0) += 1;
    };
    for &segment in &segments {
        let Coord(x1, y1) = segment.start;
        let Coord(x2, y2) = segment.end;
        if x1 == x2 {
            let x = x1;
            for y in min(y1, y2)..=max(y1, y2) {
                insert(Coord(x, y), false);
            }
        } else if y1 == y2 {
            let y = y1;
            for x in min(x1, x2)..=max(x1, x2) {
                insert(Coord(x, y), false);
            }
        } else if y2 - y1 == x2 - x1 {
            let x = min(x1, x2);
            let y = min(y1, y2);
            for d in 0..=(y2 - y1).abs() {
                insert(Coord(x + d, y + d), true);
            }
        } else if y2 - y1 == x1 - x2 {
            let x = min(x1, x2);
            let y = max(y1, y2);
            for d in 0..=(y2 - y1).abs() {
                insert(Coord(x + d, y - d), true);
            }
        } else {
            panic!("Unsupported slope");
        }
    }

    let count1 = diagram1.values().filter(|&&v| v > 1).count();
    println!("Part 1 overlap count: {}", count1);
    let count2 = diagram2.values().filter(|&&v| v > 1).count();
    println!("Part 2 overlap count: {}", count2);
}
