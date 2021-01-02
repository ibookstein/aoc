use aoc2020::aoc_input::get_input;
use std::{assert_eq, collections::VecDeque};

type Deck = VecDeque<usize>;

#[derive(Debug, Clone)]
struct Combat {
    player1: Deck,
    player2: Deck,
}

impl Combat {
    fn new(player1: Deck, player2: Deck) -> Self {
        Self { player1, player2 }
    }

    fn round(&mut self) {
        let p1_card = self.player1.pop_front().unwrap();
        let p2_card = self.player2.pop_front().unwrap();
        assert_ne!(p1_card, p2_card);

        if p1_card > p2_card {
            self.player1.push_back(p1_card);
            self.player1.push_back(p2_card);
        } else {
            self.player2.push_back(p2_card);
            self.player2.push_back(p1_card);
        }
    }

    fn is_end(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    fn finish(&mut self) {
        while !self.is_end() {
            self.round();
        }
    }

    fn winner(&self) -> &Deck {
        if !self.player1.is_empty() {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn winner_score(&self) -> usize {
        let winner = self.winner();
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum()
    }
}

fn parse_deck(lines: &[&str]) -> Deck {
    lines[1..].iter().map(|s| s.parse().unwrap()).collect()
}

fn parse_combat_game(input: &str) -> Combat {
    let lines: Vec<_> = input.lines().collect();
    let groups: Vec<_> = lines.split(|line| line.is_empty()).collect();
    assert_eq!(groups.len(), 2);

    let player1 = parse_deck(groups[0]);
    let player2 = parse_deck(groups[1]);

    Combat::new(player1, player2)
}

fn main() {
    let input = get_input(22);
    let mut game = parse_combat_game(&input);
    game.finish();
    dbg!(game.winner_score());
}
