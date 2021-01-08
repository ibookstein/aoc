use aoc2020::aoc_input::get_input;
use aoc2020::coordinates::{Coord, Delta};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum HexDir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl From<HexDir> for Delta {
    fn from(dir: HexDir) -> Self {
        match dir {
            HexDir::East => Delta(1, 0),
            HexDir::SouthEast => Delta(1, -1),
            HexDir::SouthWest => Delta(0, -1),
            HexDir::West => Delta(-1, 0),
            HexDir::NorthEast => Delta(0, 1),
            HexDir::NorthWest => Delta(-1, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

impl Color {
    fn flip(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn flip_inplace(&mut self) {
        *self = self.flip();
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

type TileMap = HashMap<Coord, Color>;

fn parse_line(line: &str) -> Vec<HexDir> {
    let mut res = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        let dir = match c {
            'e' => HexDir::East,
            'w' => HexDir::West,
            's' => match chars.next().expect("Missing second character") {
                'e' => HexDir::SouthEast,
                'w' => HexDir::SouthWest,
                _ => panic!("Invalid character"),
            },
            'n' => match chars.next().expect("Missing second character") {
                'e' => HexDir::NorthEast,
                'w' => HexDir::NorthWest,
                _ => panic!("Invalid character"),
            },
            _ => panic!("Invalid character"),
        };
        res.push(dir);
    }
    res
}

fn traverse_directions(dirs: &Vec<HexDir>) -> Coord {
    dirs.iter()
        .fold(Coord::origin(), |acc, &dir| acc + Delta::from(dir))
}

fn parse_input(input: &str) -> Vec<Coord> {
    let tiles_directions: Vec<_> = input.lines().map(parse_line).collect();
    tiles_directions.iter().map(traverse_directions).collect()
}

fn flip_tiles(coords: &[Coord]) -> TileMap {
    let mut tiles = HashMap::<Coord, Color>::new();
    for c in coords {
        tiles.entry(*c).or_default().flip_inplace()
    }
    tiles
}

const DIRS: [HexDir; 6] = [
    HexDir::East,
    HexDir::SouthEast,
    HexDir::SouthWest,
    HexDir::West,
    HexDir::NorthWest,
    HexDir::NorthEast,
];

fn adjacent_black_tiles_count(tiles: &TileMap, loc: &Coord) -> usize {
    let mut count = 0usize;
    for d in &DIRS {
        let adj = *loc + Delta::from(*d);
        if let Some(&Color::Black) = tiles.get(&adj) {
            count += 1;
        }
    }
    count
}

fn exhibit_next_day(tiles: &TileMap) -> TileMap {
    let mut next_day = TileMap::new();

    for c in tiles.keys() {
        for d in &DIRS {
            let loc = *c + Delta::from(*d);

            if next_day.contains_key(&loc) {
                continue;
            }

            let adj_count = adjacent_black_tiles_count(tiles, &loc);
            let cur_color = tiles.get(&loc).copied().unwrap_or_default();

            let flip = match cur_color {
                Color::Black => adj_count == 0 || adj_count > 2,
                Color::White => adj_count == 2,
            };

            let next_color = if flip { cur_color.flip() } else { cur_color };
            // Optimization: we can avoid saving white tiles because that's the default
            if next_color == Color::Black {
                next_day.insert(loc, next_color);
            }
        }
    }

    next_day
}

fn exhibit_nth_day(tiles: &TileMap, days: usize) -> TileMap {
    let mut current = tiles.clone();
    for _i in 0..days {
        current = exhibit_next_day(&current);
    }
    current
}

fn count_black_tiles(tiles: &TileMap) -> usize {
    tiles
        .values()
        .copied()
        .filter(|&c| c == Color::Black)
        .count()
}

fn main() {
    let input = get_input(24);
    let tiles_coordinates: Vec<_> = parse_input(&input);
    let tiles = flip_tiles(&tiles_coordinates);
    dbg!(count_black_tiles(&tiles));

    let day100 = exhibit_nth_day(&tiles, 100);
    dbg!(count_black_tiles(&day100));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_tiles_map() -> TileMap {
        let input = include_str!("day24_test_directions.txt");
        flip_tiles(&parse_input(input))
    }

    #[test]
    fn test_directions() {
        let tiles = get_test_tiles_map();
        let count = count_black_tiles(&tiles);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_exhibit_day1() {
        let tiles = get_test_tiles_map();
        let day1 = exhibit_nth_day(&tiles, 1);
        let count = count_black_tiles(&day1);
        assert_eq!(count, 15);
    }

    #[test]
    fn test_exhibit_day100() {
        let tiles = get_test_tiles_map();
        let day100 = exhibit_nth_day(&tiles, 100);
        let count = count_black_tiles(&day100);
        assert_eq!(count, 2208);
    }
}
