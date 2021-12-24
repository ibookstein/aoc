use aoc::aoc_input::get_input;
use std::{
    collections::HashMap,
    ops::{AddAssign, Mul},
};

#[derive(Debug, Clone, Copy)]
struct Part1Turn {
    idx: usize,
    roll_sum: usize,
}

#[derive(Debug, Clone, Copy)]
struct Part2Winrate {
    wins: [usize; 2],
}

impl Part2Winrate {
    fn new() -> Self {
        Self { wins: [0usize; 2] }
    }

    fn new_win(idx: usize) -> Self {
        let mut res = Self::new();
        res.wins[idx] = 1;
        res
    }

    fn max(&self) -> usize {
        self.wins.iter().copied().max().unwrap()
    }
}

impl AddAssign<Part2Winrate> for Part2Winrate {
    fn add_assign(&mut self, rhs: Part2Winrate) {
        for i in 0..self.wins.len() {
            self.wins[i] += rhs.wins[i];
        }
    }
}

impl Mul<Part2Winrate> for usize {
    type Output = Part2Winrate;

    fn mul(self, rhs: Part2Winrate) -> Self::Output {
        let mut res = rhs;
        for r in res.wins.iter_mut() {
            *r *= self;
        }
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Simulation {
    player_pos: [usize; 2],
    player_score: [usize; 2],
    turn: usize,
}

impl Simulation {
    fn new(p1_start_pos: usize, p2_start_pos: usize) -> Self {
        Self {
            player_pos: [p1_start_pos, p2_start_pos],
            player_score: [0; 2],
            turn: 0,
        }
    }

    fn cur_idx(&self) -> usize {
        self.turn % self.player_pos.len()
    }

    fn player_roll(&mut self, idx: usize, roll: usize) {
        let new_pos = (self.player_pos[idx] + roll - 1) % 10 + 1;
        self.player_pos[idx] = new_pos;
        self.player_score[idx] += new_pos;
    }

    fn part1_next_turn(&mut self) -> Part1Turn {
        let res = Part1Turn {
            idx: self.cur_idx(),
            roll_sum: 9 * self.turn + 6,
        };
        self.turn += 1;
        res
    }

    fn part1_step(&mut self) -> bool {
        let Part1Turn { idx, roll_sum } = self.part1_next_turn();
        self.player_roll(idx, roll_sum);
        self.player_score[idx] >= 1000
    }

    fn part1_run(&mut self) {
        while !self.part1_step() {}
    }

    fn part1_result(&self) -> usize {
        3 * self.turn * self.player_score[self.cur_idx()]
    }

    fn part2_step(&mut self, roll: usize) {
        let idx = self.cur_idx();
        self.turn += 1;
        self.player_roll(idx, roll);
    }

    fn part2_winrate_compute(&self, cache: &mut HashMap<Simulation, Part2Winrate>) -> Part2Winrate {
        let prev_idx = self.turn.wrapping_sub(1) % self.player_score.len();
        if self.player_score[prev_idx] >= 21 {
            return Part2Winrate::new_win(prev_idx);
        }

        let mut res = Part2Winrate::new();
        let rolls = [
            (3usize, 1usize),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1),
        ];
        for (roll_sum, count) in rolls {
            let mut new_state = *self;
            new_state.part2_step(roll_sum);
            res += count * new_state.part2_winrate(cache);
        }
        res
    }

    fn part2_winrate(&self, cache: &mut HashMap<Simulation, Part2Winrate>) -> Part2Winrate {
        let mut state_for_cache = *self;
        state_for_cache.turn %= state_for_cache.player_score.len();
        if let Some(wr) = cache.get(&state_for_cache) {
            *wr
        } else {
            let res = self.part2_winrate_compute(cache);
            cache.insert(state_for_cache, res);
            res
        }
    }

    fn part2_result(&self) -> usize {
        self.part2_winrate(&mut HashMap::new()).max()
    }
}

fn parse_startpos_line(line: &str) -> usize {
    line.split(": ").nth(1).unwrap().parse().unwrap()
}

fn main() {
    let input = get_input(2021, 21);
    let mut lines = input.lines();
    let p1_start_pos = parse_startpos_line(lines.next().unwrap());
    let p2_start_pos = parse_startpos_line(lines.next().unwrap());
    assert!(lines.next().is_none());

    let mut sim1 = Simulation::new(p1_start_pos, p2_start_pos);
    sim1.part1_run();
    println!("Part 1 result: {}", sim1.part1_result());

    let sim2 = Simulation::new(p1_start_pos, p2_start_pos);
    println!("Part 2 result: {}", sim2.part2_result());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_simulation_part1_step() {
        let mut sim = Simulation::new(4, 8);

        sim.part1_step();
        assert_eq!(sim.player_pos, [10, 8]);
        assert_eq!(sim.player_score, [10, 0]);

        sim.part1_step();
        assert_eq!(sim.player_pos, [10, 3]);
        assert_eq!(sim.player_score, [10, 3]);

        sim.part1_step();
        assert_eq!(sim.player_pos, [4, 3]);
        assert_eq!(sim.player_score, [14, 3]);
    }

    #[test]
    fn test_simulation_part1_run() {
        let mut sim = Simulation::new(4, 8);
        sim.part1_run();
        assert_eq!(sim.part1_result(), 739785);
    }

    #[test]
    fn test_simulation_part2_step() {
        let orig = Simulation::new(4, 8);

        let mut sim = orig;
        sim.part2_step(1);
        assert_eq!(sim.player_pos, [5, 8]);
        assert_eq!(sim.player_score, [5, 0]);
        sim.part2_step(3);
        assert_eq!(sim.player_pos, [5, 1]);
        assert_eq!(sim.player_score, [5, 1]);
    }

    #[test]
    fn test_simulation_part2_result() {
        let sim = Simulation::new(4, 8);
        assert_eq!(sim.part2_result(), 444356092776315);
    }

    #[test]
    fn test_simulation_part2_winrate() {
        let mut sim = Simulation::new(10, 10);
        sim.player_score = [20, 20];
        assert_eq!(sim.part2_winrate(&mut HashMap::new()).wins, [27, 0]);
    }
}
