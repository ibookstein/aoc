use aoc2020::aoc_input::get_input;
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Term(char),
    Or(Vec<Vec<usize>>),
}

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

fn parse_rules(lines: &[&str]) -> Vec<Rule> {
    let mut rules = Vec::with_capacity(lines.len());
    for line in lines {
        let mut split = line.split(": ");
        let idx = split.next().unwrap();
        let rule = split.next().unwrap();
        assert!(split.next().is_none());

        let idx = idx.parse::<usize>().unwrap();
        let rule = parse_rule(rule);
        rules.push((idx, rule));
    }
    rules.sort_by_key(|(i, _)| *i);
    assert_eq!(rules.first().unwrap().0, 0);
    assert_eq!(rules.last().unwrap().0, rules.len() - 1);
    rules.into_iter().map(|(_, rule)| rule).collect()
}

fn build_regex_recurse(rules: &[Rule], idx: usize) -> String {
    match &rules[idx] {
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

fn build_regex(rules: &[Rule]) -> String {
    format!("^{}$", build_regex_recurse(rules, 0))
}

fn main() {
    let input = get_input(19);
    let lines: Vec<_> = input.lines().collect();

    let groups: Vec<_> = lines.split(|line| line.is_empty()).collect();
    assert_eq!(groups.len(), 2);
    let rules = parse_rules(groups[0]);
    let messages = groups[1];

    let regex_str = build_regex(&rules);
    let regex = Regex::new(&regex_str).unwrap();

    let count = messages.iter().filter(|m| regex.is_match(*m)).count();
    dbg!(count);
}
