use aoc::{aoc_input::get_input, coordinates::Coord, grid::Grid};
use std::cmp::max;

#[derive(Debug, Clone, Copy)]
enum Fold {
    AlongX(isize),
    AlongY(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridLoc {
    Empty,
    Dot,
}

impl From<GridLoc> for char {
    fn from(loc: GridLoc) -> Self {
        match loc {
            GridLoc::Empty => '.',
            GridLoc::Dot => '#',
        }
    }
}

impl Default for GridLoc {
    fn default() -> Self {
        GridLoc::Empty
    }
}

#[derive(Debug, Clone)]
struct Instructions {
    coords: Vec<Coord>,
    folds: Vec<Fold>,
}

fn parse_instructions(input: &str) -> Instructions {
    let lines: Vec<_> = input.trim().lines().collect();
    let mut groups = lines.split(|line| line.is_empty());
    let coord_strs = groups.next().expect("Missing first group");
    let fold_strs = groups.next().expect("Missing second group");
    assert!(groups.next().is_none());

    let mut coords = Vec::with_capacity(coord_strs.len());
    for coord_str in coord_strs {
        let mut split = coord_str.split(',');
        let xstr = split.next().expect("Missing X");
        let ystr = split.next().expect("Missing Y");
        assert!(split.next().is_none());
        coords.push(Coord(xstr.parse().unwrap(), ystr.parse().unwrap()));
    }

    let mut folds = Vec::with_capacity(fold_strs.len());
    for fold_str in fold_strs {
        let mut split = fold_str.split(' ').last().unwrap().split('=');
        let axis = split.next().expect("Missing axis");
        let pos_str = split.next().expect("Missing position");
        let pos: isize = pos_str.parse().unwrap();

        let fold = match axis {
            "x" => Fold::AlongX(pos),
            "y" => Fold::AlongY(pos),
            _ => panic!("Invalid axis specified"),
        };
        folds.push(fold);
    }

    Instructions { coords, folds }
}

#[derive(Debug, Clone)]
struct TransparentPaper {
    grid: Grid<GridLoc>,
}

impl TransparentPaper {
    fn new(coords: &[Coord]) -> Self {
        let mut max_x = 0isize;
        let mut max_y = 0isize;
        for &Coord(x, y) in coords.iter() {
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }

        let mut grid = Grid::new(max_x as usize + 1, max_y as usize + 1);
        for c in coords.iter() {
            *grid.get_mut(*c).unwrap() = GridLoc::Dot;
        }

        Self { grid }
    }

    fn dots(&self) -> usize {
        self.grid.values().filter(|v| **v == GridLoc::Dot).count()
    }

    fn fold(&mut self, f: Fold) {
        let (new_width, new_height) = match f {
            Fold::AlongX(x) => (x as usize, self.grid.height()),
            Fold::AlongY(y) => (self.grid.width(), y as usize),
        };

        let mut new_grid = Grid::new(new_width, new_height);
        for c in self
            .grid
            .iter()
            .filter_map(|(c, v)| if *v == GridLoc::Dot { Some(c) } else { None })
        {
            let mut new_c = c;
            if c.0 >= new_width as isize {
                new_c.0 = 2 * (new_width as isize) - new_c.0;
            }
            if c.1 >= new_height as isize {
                new_c.1 = 2 * (new_height as isize) - new_c.1;
            }
            *new_grid.get_mut(new_c).unwrap() = GridLoc::Dot;
        }
        self.grid = new_grid;
    }
}

fn main() {
    let input = get_input(2021, 13);
    let instrs = parse_instructions(&input);
    let mut paper = TransparentPaper::new(&instrs.coords);
    paper.fold(instrs.folds[0]);
    println!("Dots after first fold instruction: {}", paper.dots());

    for f in &instrs.folds[1..] {
        paper.fold(*f);
    }
    println!("{}", paper.grid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
