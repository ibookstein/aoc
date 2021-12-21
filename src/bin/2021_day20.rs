use std::collections::HashMap;

use aoc::{aoc_input::get_input, coordinates::Coord};
use bitvec::prelude::*;
use itertools::iproduct;

fn parse_pixel(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid pixel"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Image {
    finite: HashMap<Coord, bool>,
    width: isize,
    height: isize,
    infinite: bool,
}

impl Image {
    fn new(finite: HashMap<Coord, bool>, width: isize, height: isize) -> Self {
        Self {
            finite,
            width: width,
            height: height,
            infinite: false,
        }
    }

    fn get(&self, c: Coord) -> bool {
        let Coord(x, y) = c;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            self.infinite
        } else {
            *self.finite.get(&c).unwrap()
        }
    }

    fn compute_new_pixel(&self, algo: &BitVec, c: Coord) -> bool {
        let Coord(x, y) = c;
        let mask = [
            Coord(x - 2, y - 2),
            Coord(x - 1, y - 2),
            Coord(x - 0, y - 2),
            Coord(x - 2, y - 1),
            Coord(x - 1, y - 1),
            Coord(x - 0, y - 1),
            Coord(x - 2, y - 0),
            Coord(x - 1, y - 0),
            Coord(x - 0, y - 0),
        ];
        let mut idx = 0usize;
        for c in mask.iter() {
            idx = (idx << 1) | self.get(*c) as usize;
        }
        algo[idx]
    }

    fn convolve(&self, algo: &BitVec) -> Self {
        let width = self.width + 2;
        let height = self.height + 2;
        let infinite = match self.infinite {
            true => *algo.last().unwrap(),
            false => *algo.first().unwrap(),
        };
        let finite: HashMap<_, _> = iproduct!(0..width, 0..height)
            .map(|(x, y)| {
                let c = Coord(x, y);
                (c, self.compute_new_pixel(algo, c))
            })
            .collect();
        Self {
            finite,
            width,
            height,
            infinite,
        }
    }

    fn lit_pixels(&self) -> usize {
        assert!(!self.infinite);
        self.finite.values().filter(|v| **v).count()
    }
}

fn parse_image(lines: &[&str]) -> Image {
    let height = lines.len();
    let width = lines[0].len();
    let mut finite = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        assert_eq!(line.len(), width);
        for (x, v) in line.chars().map(parse_pixel).enumerate() {
            finite.insert(Coord(x as isize, y as isize), v);
        }
    }
    Image::new(finite, width as isize, height as isize)
}

fn parse_input(input: &str) -> (BitVec, Image) {
    let lines: Vec<_> = input.lines().collect();

    let algo: BitVec = lines[0].chars().map(parse_pixel).collect();
    assert_eq!(algo.len(), 512);
    assert_eq!(algo[0], !algo[511]);

    assert!(lines[1].is_empty());

    (algo, parse_image(&lines[2..]))
}

fn main() {
    let input = get_input(2021, 20);

    let (algo, mut image) = parse_input(&input);
    for i in 1..=50 {
        image = image.convolve(&algo);
        match i {
            2 | 50 => {
                println!(
                    "Lit pixels after {} enhancements: {}",
                    i,
                    image.lit_pixels()
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    fn test_input() -> (BitVec, Image) {
        parse_input(include_str!("2021_day20_example_in.txt"))
    }

    fn expected_output() -> Image {
        let s = include_str!("2021_day20_example_out.txt");
        let lines: Vec<_> = s.lines().collect();
        parse_image(&lines)
    }

    #[test]
    fn test_convolve() {
        let (algo, image) = test_input();

        let actual = image.convolve(&algo);
        let expected = expected_output();
        assert_eq!(actual, expected);
    }
}
