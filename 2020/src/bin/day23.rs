use aoc2020::aoc_input::get_input;

#[derive(Debug, Clone)]
struct Game {
    cups: Vec<usize>,
    current: usize
}

impl Game {
    fn new(cup_labels: &[usize]) -> Self {
        // Assumes cup_labels contains exactly all integers 1..=cup_labels.len()
        // Input is 1-based so pad with an extra [0] entry to avoid subtracting
        // 1 everywhere
        let mut cups = vec![0; cup_labels.len() + 1];
        for w in cup_labels.windows(2) {
            cups[w[0]] = w[1];
        }
        cups[*cup_labels.last().unwrap()] = cup_labels[0];
        Self { cups, current: cup_labels[0] }
    }

    fn next_destination(&self, label: usize) -> usize {
        if label == 1 {
            (self.cups.len() - 1) as usize
        } else {
            label - 1
        }
    }

    fn do_move(&mut self) {
        let current = self.current;
        let n1 = self.cups[current];
        let n2 = self.cups[n1];
        let n3 = self.cups[n2];

        let pickup = [n1, n2, n3];
        let mut destination = self.next_destination(current);
        while pickup.contains(&destination) {
            destination = self.next_destination(destination);
        }

        let after_destination = self.cups[destination];
        let after_pickup = self.cups[n3];

        self.current = after_pickup;
        self.cups[current] = after_pickup;

        self.cups[destination] = n1;
        self.cups[n3] = after_destination;
    }

    fn do_moves(&mut self, count: usize) {
        for _i in 0..count {
            self.do_move();
        }
    }

    fn labels_after_1(&self) -> String {
        let mut s = String::with_capacity(self.cups.len() - 2);
        let mut cur = self.cups[1];

        while cur != 1 {
            s.push(std::char::from_digit(cur as u32, 10).unwrap());
            cur = self.cups[cur];
        }
        s
    }

    fn mul_two_labels_after_1(&self) -> usize {
        let a = self.cups[1];
        let b = self.cups[a];
        a * b
    }
}

fn parse_cups(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn main() {
    let input = get_input(23);
    let orig_cups = parse_cups(input.trim());

    let mut game = Game::new(orig_cups.as_slice());
    game.do_moves(100);
    dbg!(game.labels_after_1());

    let mut orig_cups = orig_cups;
    let start = *orig_cups.iter().max().unwrap() + 1;
    let range = start..start + 1_000_000 - orig_cups.len();
    orig_cups.extend(range);
    assert_eq!(orig_cups.len(), 1_000_000);
    let mut game = Game::new(orig_cups.as_slice());
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
        let mut game = Game::new(parse_cups(&input).as_slice());
        game.do_moves(100);
        assert_eq!(game.labels_after_1(), "67384529");
    }
}
