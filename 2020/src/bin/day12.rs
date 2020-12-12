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

#[derive(Debug)]
struct Simulation {
    dir: Direction,
    ship: Coord,
    waypoint: Delta,
}

impl Simulation {
    fn new() -> Simulation {
        Simulation {
            dir: Direction::Right,
            ship: Coord::origin(),
            waypoint: Delta(10, -1),
        }
    }

    fn ship_turn(&mut self, t: Turn) {
        self.dir = self.dir.turn(t);
    }

    fn ship_move(&mut self, dir: Direction, distance: isize) {
        self.ship += distance * Delta::from(dir);
    }

    fn ship_forward(&mut self, distance: isize) {
        self.ship_move(self.dir, distance);
    }

    fn ship_run(&mut self, insn: Instruction) {
        match insn {
            Instruction::Forward(n) => self.ship_forward(n),
            Instruction::Turn(t) => self.ship_turn(t),
            Instruction::Move(d, n) => self.ship_move(d, n),
        };
    }

    fn ship_run_all(&mut self, insns: &[Instruction]) {
        for insn in insns {
            self.ship_run(*insn);
        }
    }

    fn waypoint_turn(&mut self, t: Turn) {
        self.waypoint = self.waypoint.turn(t);
    }

    fn waypoint_move(&mut self, dir: Direction, distance: isize) {
        self.waypoint += distance * Delta::from(dir);
    }

    fn waypoint_forward(&mut self, distance: isize) {
        self.ship += distance * self.waypoint;
    }

    fn waypoint_run(&mut self, insn: Instruction) {
        match insn {
            Instruction::Forward(n) => self.waypoint_forward(n),
            Instruction::Turn(t) => self.waypoint_turn(t),
            Instruction::Move(d, n) => self.waypoint_move(d, n),
        };
    }

    fn waypoint_run_all(&mut self, insns: &[Instruction]) {
        for insn in insns {
            self.waypoint_run(*insn);
        }
    }

    fn distance_from_origin(&self) -> isize {
        manhattan_distance(self.ship, Coord::origin())
    }
}

fn main() {
    let input = get_input(12);
    let insns: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut sim1 = Simulation::new();
    sim1.ship_run_all(&insns);
    dbg!(sim1.distance_from_origin());

    let mut sim2 = Simulation::new();
    sim2.waypoint_run_all(&insns);
    dbg!(sim2.distance_from_origin());
}
