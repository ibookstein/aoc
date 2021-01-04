use aoc2020::aoc_input::get_input;
use std::collections::VecDeque;

type Cup = u32;
type Cups = VecDeque<Cup>;

#[derive(Debug, Clone)]
struct Game {
    cups: Cups,
    max_label: Cup,
}

impl Game {
    fn new(cups: Cups) -> Self {
        let max_label = cups.len() as Cup;
        Self { cups, max_label }
    }

    fn next_destination(&self, label: Cup) -> Cup {
        if label == 1 {
            self.max_label
        } else {
            label - 1
        }
    }

    fn do_move(&mut self) {
        let current = self.cups[0];
        self.cups.rotate_left(1);
        let pick_up: Vec<_> = self.cups.drain(..3).collect();

        let mut destination = self.next_destination(current);
        let mut destination_pos;
        loop {
            destination_pos = self.cups.iter().position(|&e| e == destination as Cup);
            if destination_pos.is_some() {
                break;
            }
            destination = self.next_destination(destination);
        }

        let insert_pos = destination_pos.unwrap() + 1;
        for i in 0..pick_up.len() {
            self.cups.insert(insert_pos + i, pick_up[i]);
        }
    }

    fn do_moves(&mut self, count: usize) {
        for _i in 0..count {
            self.do_move();

            if _i % 0x10000 == 0 {
                dbg!(_i);
            }
        }
    }

    fn labels_after_1(&self) -> String {
        let pos_after = self.cups.iter().position(|&e| e == 1).unwrap() + 1;
        let mut s = String::with_capacity(self.cups.len() - 1);

        for i in pos_after..pos_after + self.cups.len() - 1 {
            let label = self.cups[i % self.cups.len()];
            s.push(std::char::from_digit(label as u32, 10).unwrap());
        }
        s
    }

    fn mul_two_labels_after_1(&self) -> usize {
        let pos = self.cups.iter().position(|&e| e == 1).unwrap();
        let a = self.cups[(pos + 1) % self.cups.len()];
        let b = self.cups[(pos + 2) % self.cups.len()];
        a as usize * b as usize
    }
}

fn parse_cups(input: &str) -> Cups {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect()
}

fn main() {
    let input = get_input(23);
    let orig_cups = parse_cups(input.trim());

    let mut game = Game::new(orig_cups.clone());
    game.do_moves(100);
    dbg!(game.labels_after_1());

    let mut orig_cups = orig_cups;
    let start = *orig_cups.iter().max().unwrap() + 1;
    let range = start..start + 1_000_000 - orig_cups.len() as Cup;
    orig_cups.extend(range);
    assert_eq!(orig_cups.len(), 1_000_000);
    let mut game = Game::new(orig_cups);
    game.do_moves(10_000_000);
    dbg!(game.mul_two_labels_after_1());
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_labels_after_1() {
        let input = "389125467";
        let mut game = Game::new(parse_cups(&input));
        game.do_moves(100);
        assert_eq!(game.labels_after_1(), "67384529");
    }
}
