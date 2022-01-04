use aoc::aoc_input::get_input;
use num_integer::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepType {
    IdOrPush,
    PopOrReplace,
}

#[derive(Debug, Clone, Copy)]
struct HashStep {
    stype: StepType,
    c: i64,
    d: i64,
}

#[derive(Debug, Clone, Copy)]
struct HashState<'alg> {
    steps: &'alg [HashStep],
    next_step: usize,
    depth: usize,
    remaining_pops: usize,
    value: i64,
}

impl<'alg> HashState<'alg> {
    fn new(alg: &'alg HashAlg) -> Self {
        let remaining_pops = alg
            .steps
            .iter()
            .filter(|step| step.stype == StepType::PopOrReplace)
            .count();
        Self {
            steps: &alg.steps,
            next_step: 0,
            depth: 0,
            remaining_pops,
            value: 0,
        }
    }

    fn step(&mut self, input: u8) {
        let step = self.steps[self.next_step];
        let input = input as i64;
        let decision = input != self.value % 26 + step.c;
        match step.stype {
            StepType::IdOrPush => {
                self.depth += decision as usize;
                let y = decision as i64;
                self.value = (25 * y + 1) * self.value + (input + step.d) * y;
            }
            StepType::PopOrReplace => {
                match (decision, self.value) {
                    (true, 0) => self.depth += 1,
                    (false, _) => self.depth -= 1,
                    _ => {}
                }
                self.remaining_pops -= 1;
                let y = decision as i64;
                self.value = (25 * y + 1) * (self.value / 26) + (input + step.d) * y;
            }
        }
        self.next_step += 1;
    }

    #[allow(dead_code)]
    fn steps(&mut self, inputs: &[u8]) {
        for input in inputs {
            self.step(*input);
        }
    }

    fn is_zero_hash_unreachable(&self) -> bool {
        self.depth > self.remaining_pops
    }

    fn is_complete(&self) -> bool {
        self.next_step == self.steps.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Order {
    Descending,
    Ascending,
}

#[derive(Debug, Clone)]
struct HashAlg {
    steps: Vec<HashStep>,
}

impl HashAlg {
    fn new(steps: Vec<HashStep>) -> Self {
        Self { steps }
    }

    fn dfs(&self, order: Order, parent_state: &HashState, inputs: &mut Vec<u8>) -> Option<()> {
        if parent_state.is_zero_hash_unreachable() {
            return None;
        }
        if parent_state.is_complete() {
            return match parent_state.value {
                0 => Some(()),
                _ => None,
            };
        }

        let iter = match order {
            Order::Descending => [9u8, 8, 7, 6, 5, 4, 3, 2, 1],
            Order::Ascending => [1u8, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        for input in iter {
            let mut state = *parent_state;
            state.step(input);

            inputs.push(input);
            if let Some(()) = self.dfs(order, &state, inputs) {
                return Some(());
            }
            inputs.pop();
        }

        None
    }

    fn dfs_solution(&self, order: Order) -> Option<Vec<u8>> {
        let mut inputs = Vec::with_capacity(self.steps.len());
        let state = HashState::new(&self);
        self.dfs(order, &state, &mut inputs).and(Some(inputs))
    }
}

fn parse_hash_alg(input: &str) -> HashAlg {
    let lines: Vec<_> = input.trim().lines().collect();
    let (d, r) = lines.len().div_rem(&18);
    assert_eq!(r, 0);

    let mut steps = Vec::with_capacity(d);
    for chunk in lines.chunks(18) {
        let stype = match chunk[4] {
            "div z 1" => StepType::IdOrPush,
            "div z 26" => StepType::PopOrReplace,
            _ => panic!("Invalid step type"),
        };

        let c: i64 = chunk[5].strip_prefix("add x ").unwrap().parse().unwrap();
        let d: i64 = chunk[15].strip_prefix("add y ").unwrap().parse().unwrap();
        steps.push(HashStep { stype, c, d });
    }

    HashAlg::new(steps)
}

fn main() {
    let input = get_input(2021, 24);
    let problem = parse_hash_alg(&input);

    let max_solution = problem.dfs_solution(Order::Descending);
    let max_solution: String = max_solution.unwrap().iter().map(u8::to_string).collect();
    println!("Largest model number accepted by MONAD: {}", max_solution);

    let min_solution = problem.dfs_solution(Order::Ascending);
    let min_solution: String = min_solution.unwrap().iter().map(u8::to_string).collect();
    println!("Smallest model number accepted by MONAD: {}", min_solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    fn example_hash_alg() -> HashAlg {
        let steps = vec![
            HashStep {
                stype: StepType::IdOrPush,
                c: 15,
                d: 9,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 11,
                d: 1,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 10,
                d: 11,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 12,
                d: 3,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -11,
                d: 10,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 11,
                d: 5,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 14,
                d: 0,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -6,
                d: 7,
            },
            HashStep {
                stype: StepType::IdOrPush,
                c: 10,
                d: 9,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -6,
                d: 15,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -6,
                d: 4,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -16,
                d: 10,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -4,
                d: 4,
            },
            HashStep {
                stype: StepType::PopOrReplace,
                c: -2,
                d: 9,
            },
        ];

        HashAlg::new(steps)
    }

    #[test]
    fn test_hash() {
        let hash_alg = example_hash_alg();
        let mut state = HashState::new(&hash_alg);
        state.steps(&[1; 14]);
        assert!(state.is_complete());
        assert_eq!(state.value, 3118601834);

        let mut state = HashState::new(&hash_alg);
        state.steps(&[9; 14]);
        assert!(state.is_complete());
        assert_eq!(state.value, 5688781090);
    }
}
