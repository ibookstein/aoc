use aoc::aoc_input::get_input;
use itermore::IterMore;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PairInsertionRules {
    rules: HashMap<[char; 2], char>,
}

impl PairInsertionRules {
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Self, &'static str> {
        let mut rules = HashMap::new();
        for line in lines {
            let mut split = line.split(" -> ");
            let lhs = split.next().ok_or("No left-hand side")?;
            let rhs = split.next().ok_or("No right-hand side")?;

            if split.next().is_some() {
                return Err("Trailing garbage");
            }

            let mut lhs_chars = lhs.chars();
            let key = [
                lhs_chars.next().ok_or("Missing LHS char #1")?,
                lhs_chars.next().ok_or("Missing LHS char #2")?,
            ];
            if lhs_chars.next().is_some() {
                return Err("LHS trailing garbage");
            }

            let mut rhs_chars = rhs.chars();
            let val = rhs_chars.next().ok_or("Missing RHS char #1")?;
            if rhs_chars.next().is_some() {
                return Err("RHS trailing garbage");
            }

            rules.insert(key, val);
        }

        Ok(Self { rules })
    }

    fn apply(&self, ph: &PolymerHist) -> PolymerHist {
        let mut hist = HashMap::new();
        for (&[c1, c2], &v) in ph.hist.iter() {
            let insertion = *self.rules.get(&[c1, c2]).unwrap();
            *hist.entry([c1, insertion]).or_insert(0) += v;
            *hist.entry([insertion, c2]).or_insert(0) += v;
        }

        PolymerHist {
            hist,
            first: ph.first,
            last: ph.last,
        }
    }

    fn apply_n(&self, ph: &PolymerHist, n: usize) -> PolymerHist {
        if n == 0 {
            return ph.clone();
        }

        let mut res = self.apply(ph);
        for _ in 0..n - 1 {
            res = self.apply(&res);
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolymerHist {
    hist: HashMap<[char; 2], usize>,
    first: char,
    last: char,
}

impl PolymerHist {
    fn new(polymer: &str) -> Self {
        assert!(polymer.len() >= 2);
        let mut first = None;
        let mut last = '\0';
        let mut hist = HashMap::with_capacity(polymer.len() - 1);
        for [c1, c2] in polymer.chars().windows() {
            if first.is_none() {
                first = Some(c1);
            }
            last = c2;
            *hist.entry([c1, c2]).or_insert(0) += 1;
        }
        let first = first.unwrap();
        Self { hist, first, last }
    }

    fn most_common_minus_least_common(&self) -> usize {
        let mut char_hist = HashMap::new();
        for (&[c1, c2], &v) in self.hist.iter() {
            *char_hist.entry(c1).or_insert(0) += v;
            *char_hist.entry(c2).or_insert(0) += v;
        }
        *char_hist.get_mut(&self.first).unwrap() += 1;
        *char_hist.get_mut(&self.last).unwrap() += 1;

        let mut freqs: Vec<_> = char_hist.values().collect();
        freqs.sort();
        (*freqs.last().unwrap() - *freqs.first().unwrap()) / 2
    }
}

fn main() {
    let input = get_input(2021, 14);
    let mut lines = input.lines();

    let polymer_template = lines.next().unwrap().to_owned();
    assert!(lines.next().unwrap().is_empty());
    let rules = PairInsertionRules::from_lines(lines).unwrap();
    let poly_hist = PolymerHist::new(&polymer_template);

    let result1 = rules.apply_n(&poly_hist, 10);
    let diff1 = result1.most_common_minus_least_common();
    println!("Most common minus least common, part 1: {}", diff1);

    let result2 = rules.apply_n(&result1, 30);
    let diff2 = result2.most_common_minus_least_common();
    println!("Most common minus least common, part 2: {}", diff2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const POLYMER_TEMPLATE: &str = "NNCB";

    const RULE_STRINGS: &[&str] = &[
        "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
        "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
    ];

    fn get_rules() -> PairInsertionRules {
        PairInsertionRules::from_lines(RULE_STRINGS.iter().copied()).unwrap()
    }

    #[test]
    fn test_pair_insertion_rules_apply() {
        let rules = get_rules();
        let hist = PolymerHist::new(POLYMER_TEMPLATE);

        let actual = rules.apply(&hist);
        let expected = PolymerHist::new("NCNBCHB");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pair_insertion_rules_apply2() {
        let rules = get_rules();
        let hist = PolymerHist::new(&POLYMER_TEMPLATE);
        assert_eq!(hist.most_common_minus_least_common(), 1);
        let res1 = rules.apply(&hist);
        assert_eq!(res1.most_common_minus_least_common(), 1);
        let res10 = rules.apply_n(&hist, 10);
        assert_eq!(res10.most_common_minus_least_common(), 1588);
    }
}
