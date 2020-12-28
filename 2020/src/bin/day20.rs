use aoc2020::aoc_input::get_input;
use aoc2020::coordinates::Coord;
use aoc2020::grid::Grid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tile(usize, Grid<char>);
type TileMap = HashMap<usize, Tile>;

fn parse_tile(lines: &[&str]) -> Tile {
    let right = lines[0].split(' ').nth(1).unwrap();
    let right = &right[..right.find(':').unwrap()];
    let tile_number: usize = right.parse().unwrap();
    let grid: Grid<char> = lines[1..].join("\n").parse().unwrap();

    Tile(tile_number, grid)
}

fn solve_part1(tiles: &TileMap) {
    let mut facet_map = HashMap::<String, Vec<usize>>::new();
    for tile in tiles.values() {
        let g = &tile.1;
        let w = g.width() as isize;
        let h = g.height() as isize;

        let mut f1 = String::with_capacity(w as usize);
        let mut f2 = String::with_capacity(w as usize);
        for x in 0..w {
            f1.push(*g.get(Coord(x, 0)).unwrap());
            f2.push(*g.get(Coord(x, h - 1)).unwrap());
        }

        let mut f3 = String::with_capacity(h as usize);
        let mut f4 = String::with_capacity(h as usize);
        for y in 0..h {
            f3.push(*g.get(Coord(0, y)).unwrap());
            f4.push(*g.get(Coord(w - 1, y)).unwrap());
        }

        for f in vec![f1, f2, f3, f4].into_iter() {
            let rev = f.chars().rev().collect::<String>();
            facet_map.entry(f).or_default().push(tile.0);
            facet_map.entry(rev).or_default().push(tile.0);
        }
    }

    let mut tile_noadj_count = HashMap::<usize, usize>::new();
    for v in facet_map.values() {
        match v.len() {
            1 => *tile_noadj_count.entry(v[0]).or_default() += 1,
            2 => continue,
            _ => unreachable!(),
        }
    }

    let mut product = 1usize;
    for (tile_number, noadj_count) in tile_noadj_count {
        match noadj_count / 2 {
            0 | 1 => continue,
            2 => product *= tile_number,
            _ => unreachable!(),
        }
    }

    dbg!(product);
}

fn main() {
    let input = get_input(20);
    let lines: Vec<_> = input[..input.len() - 1].lines().collect();
    let groups: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let mut tiles = TileMap::new();
    for group in groups {
        let tile = parse_tile(group);
        tiles.insert(tile.0, tile);
    }

    solve_part1(&tiles);
}
