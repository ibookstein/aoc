use aoc2020::aoc_input::get_input;
use aoc2020::coordinates::{Coord, CoordN, DeltaN};
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

#[derive(Debug)]
struct PocketDim {
    dims: usize,
    dirs: Vec<DeltaN>,
    active: HashSet<CoordN>,
}

impl PocketDim {
    fn from_2d_initial_state(s: &str, dims: usize) -> Self {
        assert!(dims >= 2);
        let dirs = Self::dirs(dims);

        let grid: Grid<GridPoint> = s.parse().unwrap();
        let mut active = HashSet::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let c = Coord(x as isize, y as isize);
                if *grid.get(c).unwrap() == GridPoint::Active {
                    let mut vec = vec![0isize; dims];
                    vec[0] = x as isize;
                    vec[1] = y as isize;
                    active.insert(CoordN::from_vec(vec));
                }
            }
        }
        PocketDim { dims, dirs, active }
    }

    fn dirs(dims: usize) -> Vec<DeltaN> {
        let mut digits = vec![-1isize; dims];
        let mut dirs = Vec::<DeltaN>::with_capacity(3usize.pow(dims as u32) - 1);

        loop {
            if digits.iter().any(|d| *d != 0) {
                dirs.push(DeltaN::from_vec(digits.clone()));
            }

            if digits.iter().all(|d| *d == 1) {
                break;
            }

            for i in 0..digits.len() {
                let old = digits[i];
                if old != 1 {
                    digits[i] = old + 1;
                    break;
                }
                digits[i] = -1;
            }
        }

        dirs
    }

    fn tick(&mut self) {
        let mut candidates = HashSet::new();
        for c in self.active.iter() {
            for d in self.dirs.iter() {
                candidates.insert(c + d);
            }
        }

        let mut new_active = HashSet::new();
        for c in candidates.iter() {
            let active = self.active.contains(c);
            let neighbors_active = self
                .dirs
                .iter()
                .filter(|d| self.active.contains(&(c + *d)))
                .count();

            let next_active = match (active, neighbors_active) {
                (false, 3) => true,
                (true, 2) | (true, 3) => true,
                _ => false,
            };
            if next_active {
                new_active.insert(c.clone());
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
    let mut pd = PocketDim::from_2d_initial_state(&input, 3);
    pd.ticks(6);
    dbg!(pd.active_count());

    let mut pd = PocketDim::from_2d_initial_state(&input, 4);
    pd.ticks(6);
    dbg!(pd.active_count());
}
