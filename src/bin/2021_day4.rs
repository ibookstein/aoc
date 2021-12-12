use aoc::{aoc_input::get_input, coordinates::Coord, grid::Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Unmarked,
    Marked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkResult {
    AlreadyWon,
    WonNow,
    DidNotWinYet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BingoBoard {
    grid: Grid<(u8, Mark)>,
    last_called: Option<u8>,
    won: bool,
}

impl BingoBoard {
    fn from_lines(lines: &[&str]) -> Result<Self, &'static str> {
        let mut width: Option<usize> = None;
        let mut grid = Vec::new();

        for &line in lines {
            let nums: Result<Vec<_>, _> = line
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<u8>().map(|n| (n, Mark::Unmarked)))
                .collect();
            let nums = nums.or(Err("Invalid integer input"))?;

            if width.is_some() && width.unwrap() != nums.len() {
                return Err("Non-uniform length");
            }

            width = Some(nums.len());
            grid.extend(nums);
        }

        let width = width.ok_or("Empty input")?;
        Ok(Self {
            grid: Grid::from_vec_and_width(grid, width),
            last_called: None,
            won: false,
        })
    }

    fn marked(&self, c: Coord) -> bool {
        self.grid
            .get(c)
            .map(|(_, m)| *m == Mark::Marked)
            .unwrap_or(false)
    }

    fn mark(&mut self, n: u8) -> MarkResult {
        if self.won {
            return MarkResult::AlreadyWon;
        }

        for cell in self.grid.values_mut().filter(|(num, _)| *num == n) {
            cell.1 = Mark::Marked;
        }
        self.last_called = Some(n);

        match self.check_win() {
            false => MarkResult::DidNotWinYet,
            true => {
                self.won = true;
                MarkResult::WonNow
            }
        }
    }

    fn check_win(&self) -> bool {
        let width = self.grid.width() as isize;
        let height = self.grid.height() as isize;

        for row in 0..height {
            if (0..width).all(|x| self.marked(Coord(x, row))) {
                return true;
            }
        }

        for col in 0..width {
            if (0..height).all(|y| self.marked(Coord(col, y))) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let unmarked_sum = self
            .grid
            .values()
            .filter_map(|(n, m)| match m {
                Mark::Unmarked => Some(*n as u32),
                Mark::Marked => None,
            })
            .sum::<u32>();

        unmarked_sum * (self.last_called.unwrap() as u32)
    }
}

fn main() {
    let input = get_input(2021, 4);
    let lines: Vec<_> = input.trim().lines().collect();
    let mut groups = lines.split(|line| line.is_empty());

    let nums_str = groups.next().unwrap()[0];
    let nums: Vec<_> = nums_str
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<_> = groups.map(|g| BingoBoard::from_lines(g).unwrap()).collect();
    let mut win_order = Vec::with_capacity(boards.len());
    for &num in &nums {
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if board.mark(num) == MarkResult::WonNow {
                win_order.push(i);
            }
        }
    }

    let first = *win_order.first().unwrap();
    let last = *win_order.last().unwrap();
    println!("First winning score: {}", boards[first].score());
    println!("Last winning score: {}", boards[last].score());
}
