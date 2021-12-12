use aoc::{
    aoc_input::get_input,
    coordinates::{Coord, Direction8},
    grid::Grid,
};
use std::collections::HashSet;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EnergyLevel(u8);

impl TryFrom<char> for EnergyLevel {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(EnergyLevel(value as u8 - '0' as u8)),
            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OctopusMap {
    grid: Grid<EnergyLevel>,
}

impl OctopusMap {
    fn new(grid: Grid<EnergyLevel>) -> Self {
        Self { grid }
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn step(&mut self) -> usize {
        let mut flashed = HashSet::<Coord>::new();
        let mut dfs = Vec::<Coord>::new();

        for (coord, energy) in self.grid.iter_mut() {
            energy.0 += 1;
            if energy.0 > 9 {
                flashed.insert(coord);
                dfs.push(coord);
            }
        }

        while !dfs.is_empty() {
            let cur = dfs.pop().unwrap();
            for d in Direction8::iter() {
                let neighbor = cur + d.into();
                if flashed.contains(&neighbor) {
                    continue;
                }

                let mut neighbor_energy = match self.grid.get_mut(neighbor) {
                    None => continue,
                    Some(x) => x,
                };

                neighbor_energy.0 += 1;
                if neighbor_energy.0 > 9 {
                    flashed.insert(neighbor);
                    dfs.push(neighbor);
                }
            }
        }

        for c in &flashed {
            self.grid.get_mut(*c).unwrap().0 = 0;
        }

        flashed.len()
    }

    fn steps(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.step()).sum()
    }
}

fn main() {
    let input = get_input(2021, 11);
    let mut map = OctopusMap::new(input.parse().unwrap());

    const STEPS: usize = 100;
    println!(
        "After {} steps there have been {} flashes",
        STEPS,
        map.steps(STEPS)
    );

    let mut count = STEPS;
    let len = map.len();
    loop {
        count += 1;
        if map.step() == len {
            break;
        }
    }

    println!("Synchronized after {} steps", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
