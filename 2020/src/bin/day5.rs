use aoc2020::aoc_input::get_input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BoardingPass {
    row: usize,
    col: usize,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for BoardingPass {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_row, enc_col) = s.split_at(7);
        if enc_row.len() != 7 || enc_col.len() != 3 {
            return Err("Invalid boarding pass string length");
        };

        let bin_row = enc_row.replace('F', "0").replace('B', "1");
        let bin_col = enc_col.replace('L', "0").replace('R', "1");

        let row = usize::from_str_radix(&bin_row, 2).or(Err("Invalid row"))?;
        let col = usize::from_str_radix(&bin_col, 2).or(Err("Invalid column"))?;

        Ok(BoardingPass { row, col })
    }
}

fn main() {
    let input = get_input(5);
    let boarding_passes: HashSet<BoardingPass> =
        input.lines().map(|line| line.parse().unwrap()).collect();

    let max_seat_id = boarding_passes.iter().map(|bp| bp.seat_id()).max().unwrap();
    dbg!(max_seat_id);

    let min_row = boarding_passes.iter().map(|bp| bp.row).min().unwrap();
    let max_row = boarding_passes.iter().map(|bp| bp.row).max().unwrap();
    let min_col = 0usize;
    let max_col = 6usize;

    for r in min_row + 1..max_row {
        for c in min_col..=max_col {
            let bp = BoardingPass { row: r, col: c };
            if !boarding_passes.contains(&bp) {
                dbg!((bp, bp.seat_id()));
            }
        }
    }
}
