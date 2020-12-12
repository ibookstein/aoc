use aoc2020::aoc_input::get_input;
use aoc2020::coordinates::{manhattan_distance, Coord, Delta, Direction, Turn};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Turn(Turn),
    Forward(isize),
    Move(Direction, isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.char_indices();
        let first = chars.next().ok_or("Empty string")?.1;
        let rest_start = chars.next().ok_or("No number")?.0;
        let rest = &s[rest_start..];
        let num = isize::from_str(rest).or(Err("Invalid number"))?;

        match first {
            'N' => Ok(Instruction::Move(Direction::Up, num)),
            'S' => Ok(Instruction::Move(Direction::Down, num)),
            'E' => Ok(Instruction::Move(Direction::Right, num)),
            'W' => Ok(Instruction::Move(Direction::Left, num)),
            'L' => Ok(Instruction::Turn((num / 90) * Turn::Left)),
            'R' => Ok(Instruction::Turn((num / 90) * Turn::Right)),
            'F' => Ok(Instruction::Forward(num)),
            _ => Err("Invalid first character")?,
        }
    }
}

struct Simulation {
    dir: Direction,
    pos: Coord,
}

impl Simulation {
    fn new() -> Simulation {
        Simulation {
            dir: Direction::Right,
            pos: Coord::origin(),
        }
    }

    fn turn(&mut self, t: Turn) {
        self.dir = self.dir.turn(t);
    }

    fn dir_move(&mut self, dir: Direction, distance: isize) {
        self.pos += distance * Delta::from(dir);
    }

    fn forward(&mut self, distance: isize) {
        self.dir_move(self.dir, distance);
    }

    fn run(&mut self, insn: Instruction) {
        match insn {
            Instruction::Forward(n) => self.forward(n),
            Instruction::Turn(t) => self.turn(t),
            Instruction::Move(d, n) => self.dir_move(d, n),
        };
    }

    fn run_multiple(&mut self, insns: &[Instruction]) {
        for insn in insns {
            self.run(*insn);
        }
    }

    fn distance_from_origin(&self) -> isize {
        manhattan_distance(self.pos, Coord::origin())
    }
}

fn main() {
    let input = get_input(12);
    let insns: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut sim = Simulation::new();
    sim.run_multiple(&insns);
    dbg!(sim.distance_from_origin());
}
