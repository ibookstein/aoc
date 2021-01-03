use aoc2020::aoc_input::get_input;
use std::collections::{HashSet, VecDeque};

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
}

fn parse_deck(lines: &[&str]) -> Deck {
    lines[1..].iter().map(|s| s.parse().unwrap()).collect()
}

fn parse_combat_decks(input: &str) -> (Deck, Deck) {
    let lines: Vec<_> = input.lines().collect();
    let groups: Vec<_> = lines.split(|line| line.is_empty()).collect();
    assert_eq!(groups.len(), 2);

    let player1 = parse_deck(groups[0]);
    let player2 = parse_deck(groups[1]);

    (player1, player2)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RecursiveCombatState {
    player1: Deck,
    player2: Deck,
}

#[derive(Debug, Clone)]
struct RecursiveCombat {
    state: RecursiveCombatState,
    states: HashSet<RecursiveCombatState>,
}

impl RecursiveCombat {
    fn new(player1: Deck, player2: Deck) -> Self {
        Self {
            state: RecursiveCombatState { player1, player2 },
            states: HashSet::new(),
        }
    }

    fn round(&mut self) {
        let p1_card = self.state.player1.pop_front().unwrap();
        let p2_card = self.state.player2.pop_front().unwrap();
        assert_ne!(p1_card, p2_card);

        let winner = if p1_card <= self.state.player1.len() && p2_card <= self.state.player2.len() {
            let p1_deck: Deck = self.state.player1.iter().copied().take(p1_card).collect();
            let p2_deck: Deck = self.state.player2.iter().copied().take(p2_card).collect();

            let mut nested = Self::new(p1_deck, p2_deck);
            nested.finish();
            let nested_winner = nested.winner().unwrap();

            if std::ptr::eq(nested_winner, &nested.state.player1) {
                &self.state.player1
            } else {
                &self.state.player2
            }
        } else if p1_card > p2_card {
            &self.state.player1
        } else {
            &self.state.player2
        };

        if std::ptr::eq(winner, &self.state.player1) {
            self.state.player1.push_back(p1_card);
            self.state.player1.push_back(p2_card);
        } else {
            self.state.player2.push_back(p2_card);
            self.state.player2.push_back(p1_card);
        }
    }

    fn winner(&self) -> Option<&Deck> {
        if self.states.contains(&self.state) {
            Some(&self.state.player1)
        } else if self.state.player1.is_empty() {
            Some(&self.state.player2)
        } else if self.state.player2.is_empty() {
            Some(&self.state.player1)
        } else {
            None
        }
    }

    fn finish(&mut self) {
        while self.winner().is_none() {
            self.states.insert(self.state.clone());
            self.round();
        }
    }
}

fn score(winner: &Deck) -> usize {
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

fn main() {
    let input = get_input(22);
    let (player1, player2) = parse_combat_decks(&input);

    let mut combat = Combat::new(player1.clone(), player2.clone());
    combat.finish();
    dbg!(score(combat.winner()));

    let mut recursive_combat = RecursiveCombat::new(player1, player2);
    recursive_combat.finish();
    dbg!(score(recursive_combat.winner().unwrap()));
}
