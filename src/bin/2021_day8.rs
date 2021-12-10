use aoc::aoc_input::get_input;
use std::{collections::HashMap, str::FromStr};

fn part1(s: &str) {
    let mut total = 0usize;
    for line in s.trim().lines() {
        let output_values = line.split(" | ").nth(1).unwrap().split(' ');
        for val in output_values {
            total += match val.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }

    println!(
        "The digits 1, 4, 7, or 8 appear this many times in the output values: {}",
        total
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SigPattern {
    sigs: u8,
}

impl FromStr for SigPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self { sigs: 0 };
        for c in s.chars() {
            match c {
                'a'..='g' => {
                    let bit = 1u8 << (c as u8 - 'a' as u8);
                    if res.sigs & bit != 0 {
                        return Err("Duplicate character");
                    }
                    res.sigs |= bit;
                }
                _ => return Err("Invalid character encountered"),
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct SigEntry {
    digits: [SigPattern; 10],
    output: [SigPattern; 4],
}

impl FromStr for SigEntry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" | ");
        let digits = split.next().ok_or("Missing digits")?;
        let output = split.next().ok_or("Missing output")?;
        if split.next().is_some() {
            return Err("Trailing garbage");
        }

        let digits: Result<Vec<SigPattern>, _> = digits.split(' ').map(|s| s.parse()).collect();
        let digits: [SigPattern; 10] = digits?.try_into().or(Err("Invalid amount of digits"))?;
        let output: Result<Vec<SigPattern>, _> = output.split(' ').map(|s| s.parse()).collect();
        let output: [SigPattern; 4] = output?.try_into().or(Err("Invalid amount of output"))?;
        Ok(Self { digits, output })
    }
}

/*
2 chars => 1
3 chars => 7
4 chars => 4
5 chars => 2, 3, 5
6 chars => 0, 6, 9
7 chars => 8

Intersection between chars of:

* 2 (5 chars) and 1 (2 chars, unique) has size 1
* 3 (5 chars) and 1 (2 chars, unique) has size 2
* 5 (5 chars) and 1 (2 chars, unique) has size 1
- Uniquely detect 3

* 2 (5 chars) and 4 (4 chars, unique) has size 2
* 5 (5 chars) and 4 (4 chars, unique) has size 3
- Uniquely detect 2 and 5

* 0 (6 chars) and 1 (2 chars, unique) has size 2
* 6 (6 chars) and 1 (2 chars, unique) has size 1
* 9 (6 chars) and 1 (2 chars, unique) has size 2
- Uniquely detect 6

* 0 (6 chars) and 4 (4 chars, unique) has size 3
* 9 (6 chars) and 4 (4 chars, unique) has size 4
- Uniquely detect 0 and 9
*/

impl SigEntry {
    fn find_exact_len<'a>(
        slice: &'a [SigPattern],
        len: u32,
    ) -> impl Iterator<Item = SigPattern> + 'a {
        slice
            .iter()
            .copied()
            .filter(move |s| s.sigs.count_ones() == len)
    }

    fn find_single_exact_len(slice: &[SigPattern], len: u32) -> Option<SigPattern> {
        let mut iter = Self::find_exact_len(slice, len);
        let res = iter.next();
        if iter.next().is_none() {
            res
        } else {
            None
        }
    }

    fn find_by_intersection_len<'a>(
        slice: &'a [SigPattern],
        other: SigPattern,
        intersection_len: u32,
    ) -> impl Iterator<Item = SigPattern> + 'a {
        slice
            .iter()
            .copied()
            .filter(move |s| (s.sigs & other.sigs).count_ones() == intersection_len)
    }

    fn find_single_by_intersection_len(
        slice: &[SigPattern],
        other: SigPattern,
        intersection_len: u32,
    ) -> Option<SigPattern> {
        let mut iter = Self::find_by_intersection_len(slice, other, intersection_len);
        let res = iter.next();
        if iter.next().is_none() {
            res
        } else {
            None
        }
    }

    fn solve(&self) -> Option<u32> {
        let one = Self::find_single_exact_len(&self.digits, 2)?;
        let seven = Self::find_single_exact_len(&self.digits, 3)?;
        let four = Self::find_single_exact_len(&self.digits, 4)?;
        let eight = Self::find_single_exact_len(&self.digits, 7)?;

        let mut len5: Vec<_> = Self::find_exact_len(&self.digits, 5).collect();
        let three = Self::find_single_by_intersection_len(&len5, one, 2)?;
        len5.retain(|&x| x != three);
        let two = Self::find_single_by_intersection_len(&len5, four, 2)?;
        let five = Self::find_single_by_intersection_len(&len5, four, 3)?;

        let mut len6: Vec<_> = Self::find_exact_len(&self.digits, 6).collect();
        let six = Self::find_single_by_intersection_len(&len6, one, 1)?;
        len6.retain(|&x| x != six);
        let zero = Self::find_single_by_intersection_len(&len6, four, 3)?;
        let nine = Self::find_single_by_intersection_len(&len6, four, 4)?;

        let map = [
            (zero, 0u32),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ];

        let map = HashMap::<_, _>::from_iter(map.into_iter());
        let res = 1000 * *map.get(&self.output[0]).unwrap()
            + 100 * *map.get(&self.output[1]).unwrap()
            + 10 * *map.get(&self.output[2]).unwrap()
            + *map.get(&self.output[3]).unwrap();
        Some(res)
    }
}

fn part2(s: &str) {
    let total: u32 = s
        .trim()
        .lines()
        .map(|s| s.parse::<SigEntry>().unwrap().solve().unwrap())
        .sum();
    println!("Sum of all the output values: {}", total);
}

fn main() {
    let input = get_input(2021, 8);

    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
