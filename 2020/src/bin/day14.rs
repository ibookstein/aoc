use aoc2020::aoc_input::get_input;
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Mask {
    and: u64,
    or: u64,
}

impl Mask {
    fn default() -> Mask {
        Mask {
            and: (1 << 36) - 1,
            or: 0,
        }
    }

    fn mask(&self, data: u64) -> u64 {
        data & self.and | self.or
    }

    fn addresses(&self, addr: u64) -> Vec<u64> {
        let floating_bits = !(self.or | !self.and);
        let overwritten = addr | self.or;
        let mut addresses = vec![overwritten & !floating_bits];
        for i in 0..64 {
            let bit = 1u64 << i;
            if bit & floating_bits == 0 {
                continue;
            }

            let prev_len = addresses.len();
            for i in 0..prev_len {
                addresses.push(addresses[i] | bit)
            }
        }
        addresses
    }
}

impl FromStr for Mask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Mask::default();
        for (i, c) in s.chars().rev().enumerate() {
            let bit = 1u64 << i;
            match c {
                '0' => mask.and &= !bit,
                '1' => mask.or |= bit,
                'X' => (),
                _ => return Err("Invalid character in mask string"),
            };
        }
        Ok(mask)
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    SetMask { mask: Mask },
    Store { addr: u64, data: u64 },
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" = ");
        let lhs = split.next().ok_or("No left-hand side")?;
        let rhs = split.next().ok_or("No right-hand side")?;

        if lhs == "mask" {
            let mask: Mask = rhs.parse()?;
            return Ok(Instruction::SetMask { mask });
        }

        const MEM_PREFIX: &str = "mem[";
        const MEM_SUFFIX: &str = "]";
        if lhs.starts_with(MEM_PREFIX) && lhs.ends_with(MEM_SUFFIX) {
            let start_idx = MEM_PREFIX.len();
            let end_idx = lhs.len() - MEM_SUFFIX.len();
            let addr: u64 = lhs[start_idx..end_idx].parse().or(Err("Invalid address"))?;
            let data: u64 = rhs.parse().or(Err("Invalid data"))?;
            return Ok(Instruction::Store { addr, data });
        }

        Err("Invalid left-hand side")
    }
}

#[derive(Debug, Copy, Clone)]
enum Version {
    One,
    Two,
}

struct Machine {
    version: Version,
    mem: HashMap<u64, u64>,
    mask: Mask,
}

impl Machine {
    fn new(version: Version) -> Machine {
        Machine {
            version,
            mem: HashMap::new(),
            mask: Mask::default(),
        }
    }

    fn set_mask(&mut self, mask: Mask) {
        self.mask = mask;
    }

    fn store_v1(&mut self, addr: u64, data: u64) {
        self.mem.insert(addr, self.mask.mask(data));
    }

    fn store_v2(&mut self, addr: u64, data: u64) {
        for addr in self.mask.addresses(addr) {
            self.mem.insert(addr, data);
        }
    }

    fn store(&mut self, addr: u64, data: u64) {
        match self.version {
            Version::One => self.store_v1(addr, data),
            Version::Two => self.store_v2(addr, data),
        }
    }

    fn run(&mut self, insn: Instruction) {
        match insn {
            Instruction::SetMask { mask } => self.set_mask(mask),
            Instruction::Store { addr, data } => self.store(addr, data),
        }
    }

    fn run_multi(&mut self, insns: &[Instruction]) {
        for insn in insns {
            self.run(*insn);
        }
    }

    fn mem_sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

fn main() {
    let input = get_input(14);
    let insns: Vec<Instruction> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut v1 = Machine::new(Version::One);
    v1.run_multi(&insns);
    dbg!(v1.mem_sum());

    let mut v2 = Machine::new(Version::Two);
    v2.run_multi(&insns);
    dbg!(v2.mem_sum());
}
