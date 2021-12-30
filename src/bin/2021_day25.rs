use aoc::{aoc_input::get_input, coordinates::Coord, grid::Grid};
use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    East,
    South,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '>' => Ok(Tile::East),
            'v' => Ok(Tile::South),
            _ => Err("Invalid character"),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::East => '>',
            Tile::South => 'v',
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

#[derive(Debug, Clone)]
struct Simulation {
    grid: Grid<Tile>,
    steps: usize,
}

impl Simulation {
    fn new(grid: Grid<Tile>) -> Self {
        Self { grid, steps: 0 }
    }

    fn step(&mut self) -> bool {
        let (w, h) = (self.grid.width() as isize, self.grid.height() as isize);
        let mut any_moved = false;

        let mut move_east = Vec::new();
        for (x, y) in iproduct!(0..w, 0..h) {
            let (cur, next) = (Coord(x, y), Coord((x + 1) % w, y));
            let cur_tile = *self.grid.get(cur).unwrap();
            let next_tile = *self.grid.get(next).unwrap();
            if (cur_tile, next_tile) == (Tile::East, Tile::Empty) {
                move_east.push((cur, next));
            }
        }

        for (cur, next) in move_east {
            *self.grid.get_mut(cur).unwrap() = Tile::Empty;
            *self.grid.get_mut(next).unwrap() = Tile::East;
            any_moved = true;
        }

        let mut move_south = Vec::new();
        for (x, y) in iproduct!(0..w, 0..h) {
            let (cur, next) = (Coord(x, y), Coord(x, (y + 1) % h));
            let cur_tile = *self.grid.get(cur).unwrap();
            let next_tile = *self.grid.get(next).unwrap();

            if (cur_tile, next_tile) == (Tile::South, Tile::Empty) {
                move_south.push((cur, next));
            }
        }

        for (cur, next) in move_south {
            *self.grid.get_mut(cur).unwrap() = Tile::Empty;
            *self.grid.get_mut(next).unwrap() = Tile::South;
            any_moved = true;
        }

        self.steps += 1;
        any_moved
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn main() {
    let input = get_input(2021, 25);
    let grid: Grid<Tile> = input.parse().unwrap();

    let mut sim = Simulation::new(grid);
    sim.run();
    println!("Simulation stops after step: {}", sim.steps);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const INITIAL_STATE: &str = "v...>>.vv>\n\
                                 .vv>>.vv..\n\
                                 >>.>v>...v\n\
                                 >>v>>.>.v.\n\
                                 v>v.vv.v..\n\
                                 >.>>..v...\n\
                                 .vv..>.>v.\n\
                                 v.v..>>v.v\n\
                                 ....v..v.>";

    #[test]
    fn test_simulation() {
        let grid: Grid<Tile> = INITIAL_STATE.parse().unwrap();
        let mut sim = Simulation::new(grid);
        sim.run();
        assert_eq!(sim.steps, 58);
    }
}
