use aoc2020::aoc_input::get_input;
use aoc2020::grid::Grid;
use aoc2020::coordinates::{Coord, Delta};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Position {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Position::Floor),
            'L' => Ok(Position::Empty),
            '#' => Ok(Position::Occupied),
            _ => Err("Invalid char"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Clone)]
struct Simulation {
    grid: Grid<Position>,
}

impl Simulation {
    fn new(grid: Grid<Position>) -> Simulation {
        Simulation { grid }
    }

    fn decide1(&self, c: Coord) -> Option<Position> {
        let dirs = [
            Delta(-1, 0),
            Delta(0, -1),
            Delta(1, 0),
            Delta(0, 1),
            Delta(-1, -1),
            Delta(1, -1),
            Delta(-1, 1),
            Delta(1, 1),
        ];

        let occupied = dirs
            .iter()
            .filter(|d| self.grid.get(c + **d) == Some(&Position::Occupied))
            .count();
        let current = *self.grid.get(c).unwrap();

        if current == Position::Empty && occupied == 0 {
            Some(Position::Occupied)
        } else if current == Position::Occupied && occupied >= 4 {
            Some(Position::Empty)
        } else {
            None
        }
    }

    fn seat_in_direction(&self, c: Coord, dir: Delta) -> Option<Coord> {
        for scale in 1isize.. {
            let delta = scale * dir;
            let coord = c + delta;
            match self.grid.get(coord).to_owned() {
                None => return None,
                Some(Position::Floor) => continue,
                Some(_) => return Some(coord),
            };
        }
        unreachable!();
    }

    fn decide2(&self, c: Coord) -> Option<Position> {
        let dirs = [
            Delta(-1, 0),
            Delta(0, -1),
            Delta(1, 0),
            Delta(0, 1),
            Delta(-1, -1),
            Delta(1, -1),
            Delta(-1, 1),
            Delta(1, 1),
        ];

        let seats: Vec<_> = dirs
            .iter()
            .filter_map(|d| self.seat_in_direction(c, *d))
            .collect();

        let occupied = seats
            .iter()
            .filter(|c| self.grid.get(**c) == Some(&Position::Occupied))
            .count();
        let current = *self.grid.get(c).unwrap();

        if current == Position::Empty && occupied == 0 {
            Some(Position::Occupied)
        } else if current == Position::Occupied && occupied >= 5 {
            Some(Position::Empty)
        } else {
            None
        }
    }

    fn tick(&mut self, part: Part) -> bool {
        let mut next_grid = self.grid.clone();
        let mut changed = false;

        for x in 0..next_grid.width() {
            for y in 0..next_grid.height() {
                let coord = Coord(x as isize, y as isize);
                let res = match part {
                    Part::One => self.decide1(coord),
                    Part::Two => self.decide2(coord),
                };
                if let Some(p) = res {
                    changed = true;
                    *next_grid.get_mut(coord).unwrap() = p;
                }
            }
        }

        self.grid = next_grid;
        changed
    }

    fn run1(&mut self) {
        while self.tick(Part::One) {}
    }

    fn run2(&mut self) {
        while self.tick(Part::Two) {}
    }

    fn count_of(&self, p: Position) -> usize {
        self.grid.iter().filter(|pos| **pos == p).count()
    }
}

fn main() {
    let input = get_input(11);
    let grid: Grid<Position> = input.parse().unwrap();

    let mut sim1 = Simulation::new(grid);
    let mut sim2 = sim1.clone();

    sim1.run1();
    dbg!(sim1.count_of(Position::Occupied));

    sim2.run2();
    dbg!(sim2.count_of(Position::Occupied));
}
