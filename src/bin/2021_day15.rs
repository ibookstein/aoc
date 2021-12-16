use aoc::{
    aoc_input::get_input,
    coordinates::{Coord, Direction},
    grid::Grid,
};
use bimap::BiHashMap;
use num_integer::div_rem;
use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy)]
struct Risk(u32);

impl TryFrom<char> for Risk {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1'..='9' => Ok(Risk(value as u32 - '0' as u32)),
            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, Clone)]
struct CaveRiskGraph {
    g: DiGraph<(), u32>,
    lookup: BiHashMap<Coord, NodeIndex>,
    bottom_right: Coord,
}

impl CaveRiskGraph {
    fn total_risk(&self) -> u32 {
        let start = *self.lookup.get_by_left(&Coord::origin()).unwrap();
        let goal = *self.lookup.get_by_left(&self.bottom_right).unwrap();
        let node_map = dijkstra(&self.g, start, Some(goal), |e| *e.weight());
        assert_eq!(
            node_map
                .get(&self.lookup.get_by_left(&Coord(1, 0)).unwrap())
                .unwrap(),
            &1
        );
        *node_map.get(&goal).unwrap()
    }

    fn from_grid_and_cycle(grid: &Grid<Risk>, cycle: usize) -> Self {
        assert_ne!(cycle, 0);
        let mut g = DiGraph::new();
        let mut lookup = BiHashMap::<Coord, NodeIndex>::new();

        let orig_width = grid.width();
        let orig_height = grid.height();
        let width = orig_width * cycle;
        let height = orig_height * cycle;
        let bottom_right = Coord((width - 1) as isize, (height - 1) as isize);

        for y in 0..height {
            for x in 0..width {
                let c = Coord(x as isize, y as isize);
                let n = g.add_node(());
                lookup.insert_no_overwrite(c, n).unwrap();
            }
        }

        for (c, c_idx) in lookup.iter() {
            for d in Direction::iter() {
                let neighbor_coord = *c + d.into();
                let Coord(neighbor_x, neighbor_y) = neighbor_coord;
                if neighbor_x < 0
                    || neighbor_x >= width as isize
                    || neighbor_y < 0
                    || neighbor_y >= height as isize
                {
                    continue;
                }

                let (q_x, neighbor_orig_x) = div_rem(neighbor_x, orig_width as isize);
                let (q_y, neighbor_orig_y) = div_rem(neighbor_y, orig_height as isize);
                let neighbor_orig_coord = Coord(neighbor_orig_x, neighbor_orig_y);
                let neighbor_orig_risk = grid.get(neighbor_orig_coord).unwrap().0;

                let neighbor_risk = (neighbor_orig_risk - 1 + q_x as u32 + q_y as u32) % 9 + 1;
                let neighbor_idx = *lookup.get_by_left(&neighbor_coord).unwrap();
                g.add_edge(*c_idx, neighbor_idx, neighbor_risk);
            }
        }

        Self {
            g,
            lookup,
            bottom_right,
        }
    }
}

fn main() {
    let input = get_input(2021, 15);
    let grid: Grid<Risk> = input.trim().parse().unwrap();

    let g1 = CaveRiskGraph::from_grid_and_cycle(&grid, 1);
    println!("Total risk (1x1): {}", g1.total_risk());

    let g2 = CaveRiskGraph::from_grid_and_cycle(&grid, 5);
    println!("Total risk (5x5): {}", g2.total_risk());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const GRAPH_STR: &str = "1163751742\n\
                             1381373672\n\
                             2136511328\n\
                             3694931569\n\
                             7463417111\n\
                             1319128137\n\
                             1359912421\n\
                             3125421639\n\
                             1293138521\n\
                             2311944581";

    #[test]
    fn test_total_risk_cycle1() {
        let grid: Grid<Risk> = GRAPH_STR.parse().unwrap();
        let g = CaveRiskGraph::from_grid_and_cycle(&grid, 1);
        assert_eq!(g.total_risk(), 40);
    }

    #[test]
    fn test_total_risk_cycle5() {
        let grid: Grid<Risk> = GRAPH_STR.parse().unwrap();
        let g = CaveRiskGraph::from_grid_and_cycle(&grid, 5);
        assert_eq!(g.total_risk(), 315);
    }
}
