use aoc2020::aoc_input::get_input;
use aoc2020::coordinates::{Coord, Coord3, Delta3};
use aoc2020::grid::Grid;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
enum GridPoint {
    Active,
    Inactive,
}

impl TryFrom<char> for GridPoint {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(GridPoint::Inactive),
            '#' => Ok(GridPoint::Active),
            _ => Err("Invalid char"),
        }
    }
}

const DIRS: [Delta3; 26] = [
    Delta3(-1, -1, -1),
    Delta3(-1, -1, 0),
    Delta3(-1, -1, 1),
    Delta3(-1, 0, -1),
    Delta3(-1, 0, 0),
    Delta3(-1, 0, 1),
    Delta3(-1, 1, -1),
    Delta3(-1, 1, 0),
    Delta3(-1, 1, 1),
    Delta3(0, -1, -1),
    Delta3(0, -1, 0),
    Delta3(0, -1, 1),
    Delta3(0, 0, -1),
    // Delta3(0, 0, 0),
    Delta3(0, 0, 1),
    Delta3(0, 1, -1),
    Delta3(0, 1, 0),
    Delta3(0, 1, 1),
    Delta3(1, -1, -1),
    Delta3(1, -1, 0),
    Delta3(1, -1, 1),
    Delta3(1, 0, -1),
    Delta3(1, 0, 0),
    Delta3(1, 0, 1),
    Delta3(1, 1, -1),
    Delta3(1, 1, 0),
    Delta3(1, 1, 1),
];

#[derive(Debug)]
struct PocketDim {
    active: HashSet<Coord3>,
}

impl PocketDim {
    fn from_2d_initial_state(s: &str) -> Self {
        let grid: Grid<GridPoint> = s.parse().unwrap();
        let mut active = HashSet::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let c = Coord(x as isize, y as isize);
                if *grid.get(c).unwrap() == GridPoint::Active {
                    active.insert(Coord3(x as isize, y as isize, 0));
                }
            }
        }
        PocketDim { active }
    }

    fn tick(&mut self) {
        let mut candidates = HashSet::new();
        for c in self.active.iter() {
            for d in DIRS.iter() {
                candidates.insert(*c + *d);
            }
        }

        let mut new_active = HashSet::new();
        for c in candidates.iter() {
            let active = self.active.contains(c);
            let neighbors_active = DIRS
                .iter()
                .filter(|d| self.active.contains(&(*c + **d)))
                .count();

            let next_active = match (active, neighbors_active) {
                (false, 3) => true,
                (true, 2) | (true, 3) => true,
                _ => false,
            };
            if next_active {
                new_active.insert(*c);
            }
        }
        self.active = new_active;
    }

    fn ticks(&mut self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn active_count(&self) -> usize {
        self.active.len()
    }
}

fn main() {
    let input = get_input(17);
    let mut pd = PocketDim::from_2d_initial_state(&input);
    pd.ticks(6);
    dbg!(pd.active_count());
}
