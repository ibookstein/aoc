use std::{collections::HashSet, str::FromStr};

use aoc::{
    aoc_input::get_input,
    coordinates::{Coord, Direction},
    grid::Grid,
};
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HeightVal(u8);

impl TryFrom<char> for HeightVal {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(HeightVal(value as u8 - '0' as u8)),
            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeightMap {
    grid: Grid<HeightVal>,
    low_points: Vec<Coord>,
}

impl HeightMap {
    fn is_low_point(grid: &Grid<HeightVal>, c: Coord) -> bool {
        let height = *grid.get(c).unwrap();

        for d in Direction::iter() {
            let neighbor = c + d.into();
            if let Some(neighbor_height) = grid.get(neighbor) {
                if *neighbor_height <= height {
                    return false;
                }
            }
        }

        true
    }

    fn find_low_points(grid: &Grid<HeightVal>) -> Vec<Coord> {
        let mut res = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let c = Coord(x as isize, y as isize);
                if Self::is_low_point(&grid, c) {
                    res.push(c);
                }
            }
        }
        res
    }

    fn basin_size(grid: &Grid<HeightVal>, low_point: Coord) -> usize {
        let mut visited = HashSet::new();
        visited.insert(low_point);
        let mut dfs_stack = vec![low_point];

        while !dfs_stack.is_empty() {
            let cur = dfs_stack.pop().unwrap();
            let cur_height = grid.get(cur).unwrap();

            for d in Direction::iter() {
                let neighbor = cur + d.into();

                let neighbor_height = match grid.get(neighbor) {
                    None => continue,
                    Some(h) => h,
                };

                if visited.contains(&neighbor)
                    || neighbor_height < cur_height
                    || neighbor_height == &HeightVal(9)
                {
                    continue;
                }

                visited.insert(neighbor);
                dfs_stack.push(neighbor);
            }
        }

        visited.len()
    }

    fn new(grid: Grid<HeightVal>) -> Self {
        let low_points = Self::find_low_points(&grid);
        Self { grid, low_points }
    }

    fn sum_low_points_risk(&self) -> u32 {
        self.low_points
            .iter()
            .map(|h| self.grid.get(*h).unwrap().0 as u32 + 1)
            .sum()
    }

    fn sorted_basin_sizes(&self) -> Vec<usize> {
        let mut vec: Vec<_> = self
            .low_points
            .iter()
            .map(|p| Self::basin_size(&self.grid, *p))
            .collect();
        vec.sort();
        vec
    }

    fn product_top_three_basin_sizes(&self) -> usize {
        let basin_sizes: Vec<_> = self.sorted_basin_sizes();
        basin_sizes[basin_sizes.len() - 3..].iter().product()
    }
}

impl FromStr for HeightMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<HeightVal> = s.parse()?;
        Ok(HeightMap::new(grid))
    }
}

fn main() {
    let input = get_input(2021, 9);
    let map: HeightMap = input.parse().unwrap();
    println!(
        "Sum of risk levels of all low points: {}",
        map.sum_low_points_risk()
    );
    println!(
        "Product of top three basin sizes: {}",
        map.product_top_three_basin_sizes()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    static EXAMPLE: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn test_basin_sizes() {
        let map: HeightMap = EXAMPLE.parse().unwrap();
        let sizes = map.sorted_basin_sizes();
        assert_eq!(&sizes, &[3, 9, 9, 14]);
        assert_eq!(map.product_top_three_basin_sizes(), 1134);
    }
}
