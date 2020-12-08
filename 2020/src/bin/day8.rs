use aoc2020::aoc_input::get_input;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Opcode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Opcode::Nop),
            "acc" => Ok(Opcode::Acc),
            "jmp" => Ok(Opcode::Jmp),
            _ => Err("Invalid opcode string"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Insn {
    op: Opcode,
    opnd: isize,
}

impl Insn {
    fn flip(&self) -> Option<Insn> {
        match self.op {
            Opcode::Acc => None,
            Opcode::Nop => Some(Insn {
                op: Opcode::Jmp,
                opnd: self.opnd,
            }),
            Opcode::Jmp => Some(Insn {
                op: Opcode::Nop,
                opnd: self.opnd,
            }),
        }
    }
}

impl FromStr for Insn {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let opstr = match split.next() {
            None => return Err("No opcode string"),
            Some(v) => v,
        };
        let opndstr = match split.next() {
            None => return Err("No opcode operand"),
            Some(v) => v,
        };

        let op: Opcode = opstr.parse()?;
        let opnd: isize = opndstr.parse().or(Err("Failed parsing operand"))?;

        Ok(Insn { op, opnd })
    }
}

struct Regs {
    ip: isize,
    acc: isize,
}

impl Regs {
    fn new() -> Regs {
        Regs { ip: 0, acc: 0 }
    }
}

struct Machine {
    code: Vec<Insn>,
    regs: Regs,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RunResult {
    InfiniteLoop(isize),
    Termination(isize),
}

impl Machine {
    fn new(code: Vec<Insn>) -> Machine {
        let regs = Regs::new();
        Machine { code, regs }
    }

    fn tick(&mut self) {
        let insn = self.code[self.regs.ip as usize];
        match insn.op {
            Opcode::Nop => self.regs.ip += 1,
            Opcode::Jmp => self.regs.ip += insn.opnd,
            Opcode::Acc => {
                self.regs.acc += insn.opnd;
                self.regs.ip += 1;
            }
        };
    }

    fn run_once(&mut self) -> RunResult {
        let mut visited = vec![false; self.code.len()];
        loop {
            visited[self.regs.ip as usize] = true;
            self.tick();

            let ip = self.regs.ip as usize;
            if ip == self.code.len() {
                return RunResult::Termination(self.regs.acc);
            } else if visited[ip] {
                return RunResult::InfiniteLoop(self.regs.acc);
            }
        }
    }
}

fn main() {
    let input = get_input(8);
    let code: Vec<_> = input.lines().map(|s| s.parse::<Insn>().unwrap()).collect();

    let mut machine = Machine::new(code.clone());
    dbg!(machine.run_once());

    for i in 0..code.len() {
        let flipped = match code[i].flip() {
            None => continue,
            Some(insn) => insn,
        };
        let mut modified = code.clone();
        modified[i] = flipped;
        match Machine::new(modified).run_once() {
            RunResult::InfiniteLoop(_) => continue,
            RunResult::Termination(acc) => {
                println!("Termination accumulator: {}", acc);
                break;
            }
        }
    }
}
