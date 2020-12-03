use aoc2020::aoc_input::get_input;
use std::str::FromStr;

struct Policy {
    min: usize,
    max: usize,
    ch: char,
}

impl Policy {
    fn check1(&self, s: &str) -> bool {
        let count = s.chars().filter(|ch| *ch == self.ch).count();
        self.min <= count && count <= self.max
    }

    fn check2(&self, s: &str) -> bool {
        let ch1 = s.chars().nth(self.min - 1).expect("Bad low index");
        let ch2 = s.chars().nth(self.max - 1).expect("Bad high index");

        (ch1 == self.ch) ^ (ch2 == self.ch)
    }
}

impl FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let (range, ch) = match &parts[..] {
            [range, ch] => (*range, *ch),
            _ => return Err("Bad policy format"),
        };

        let ch = ch.parse::<char>().or(Err("Bad policy character"))?;
        let range: Vec<_> = range.split('-').collect();
        let (min, max) = match &range[..] {
            [min, max] => (*min, *max),
            _ => return Err("Bad policy range"),
        };

        let min = min.parse::<usize>().or(Err("Bad policy range min"))?;
        let max = max.parse::<usize>().or(Err("Bad policy range max"))?;
        Ok(Policy { min, max, ch })
    }
}

fn parse_line(s: &str) -> (Policy, &str) {
    let parts: Vec<_> = s.split(": ").collect();
    let (policy, password) = match &parts[..] {
        [policy, password] => (*policy, *password),
        _ => panic!("Invalid line"),
    };
    (policy.parse().expect("Error parsing policy"), password)
}

fn main() {
    let input = get_input(2);
    let entries: Vec<_> = input.lines().map(parse_line).collect();

    let valid1 = entries
        .iter()
        .filter(|(policy, password)| policy.check1(password))
        .count();
    dbg!(valid1);

    let valid2 = entries
        .iter()
        .filter(|(policy, password)| policy.check2(password))
        .count();
    dbg!(valid2);
}
