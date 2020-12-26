use aoc2020::aoc_input::get_input;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Term(char),
    Or(Vec<Vec<usize>>),
}

type Rules = HashMap<usize, Rule>;

fn parse_rule(s: &str) -> Rule {
    match s.chars().next().unwrap() {
        '"' => {
            let ch = &s[1..s.len() - 1];
            assert!(ch.len() == 1);
            Rule::Term(ch.chars().next().unwrap())
        }
        _ => {
            let mut or = Vec::new();
            for sub in s.split(" | ") {
                let indices: Vec<usize> = sub.split(' ').map(|s| s.parse().unwrap()).collect();
                or.push(indices);
            }
            Rule::Or(or)
        }
    }
}

fn parse_rules(lines: &[&str]) -> Rules {
    let mut rules = HashMap::new();
    for line in lines {
        let mut split = line.split(": ");
        let idx = split.next().unwrap();
        let rule = split.next().unwrap();
        assert!(split.next().is_none());

        let idx = idx.parse::<usize>().unwrap();
        let rule = parse_rule(rule);
        rules.insert(idx, rule);
    }
    rules
}

fn build_regex_recurse(rules: &Rules, idx: usize) -> String {
    match rules.get(&idx).unwrap() {
        Rule::Term(c) => String::from(*c),
        Rule::Or(or) => {
            let mut clauses = Vec::new();
            for clause in or {
                let mut s = String::new();
                for i in clause {
                    s.push_str(&build_regex_recurse(rules, *i));
                }
                clauses.push(s);
            }
            format!("({})", clauses.join("|"))
        }
    }
}

fn build_regex(rules: &Rules) -> String {
    format!("^{}$", build_regex_recurse(rules, 0))
}

fn solve_part1(rules: &Rules, messages: &[&str]) {
    let regex_str = build_regex(&rules);
    let regex = Regex::new(&regex_str).unwrap();
    let count = messages.iter().filter(|m| regex.is_match(*m)).count();
    dbg!(count);
}

fn find_adjacent_matches(r: &Regex, s: &str) -> Vec<usize> {
    let mut remainder = s;
    let mut res = Vec::new();
    let mut cur_end = 0usize;
    while let Some(m) = r.find(remainder) {
        cur_end += m.end();
        res.push(cur_end);
        remainder = &remainder[m.end()..];
    }
    res
}

fn check_message_part2(rule42: &Regex, rule31: &Regex, message: &str) -> bool {
    let rule42_matches = find_adjacent_matches(rule42, message);
    if rule42_matches.len() < 2 {
        return false;
    }

    // Backtrack
    for (i, m) in rule42_matches[1..].iter().rev().enumerate() {
        let remainder = &message[*m..];

        let rule31_matches = find_adjacent_matches(rule31, remainder);
        if rule31_matches.is_empty() {
            continue;
        }

        let rule31_end = *rule31_matches.last().unwrap();
        if rule31_end != remainder.len() {
            continue;
        }

        if rule42_matches.len() - i > rule31_matches.len() {
            return true;
        }
    }

    false
}

fn solve_part2(rules: &Rules, messages: &[&str]) {
    let rule42 = format!("^{}", build_regex_recurse(rules, 42));
    let rule42 = Regex::new(&rule42).unwrap();
    let rule31 = format!("^{}", build_regex_recurse(rules, 31));
    let rule31 = Regex::new(&rule31).unwrap();

    let count = messages
        .iter()
        .filter(|m| check_message_part2(&rule42, &rule31, m))
        .count();
    dbg!(count);
}

fn main() {
    let input = get_input(19);
    let lines: Vec<_> = input.lines().collect();

    let groups: Vec<_> = lines.split(|line| line.is_empty()).collect();
    assert_eq!(groups.len(), 2);
    let rules = parse_rules(groups[0]);
    let messages = groups[1];

    solve_part1(&rules, messages);
    solve_part2(&rules, messages);
}
