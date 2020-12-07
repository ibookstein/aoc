use aoc2020::aoc_input::get_input;
use std::collections::{HashMap, HashSet};

struct Rules {
    contained_by: HashMap<String, Vec<String>>,
    contains: HashMap<String, Vec<(usize, String)>>,
}

fn parse_contained_item(s: &str) -> (usize, String) {
    let split_pos = s.find(" bag").expect("Invalid rule item");
    let stripped = &s[..split_pos];
    let num_sep_pos = stripped.find(' ').expect("Invalid rule item");

    let num = &stripped[..num_sep_pos];
    let num = num.parse::<usize>().expect("Invalid rule item number");
    let color = stripped[num_sep_pos + 1..].to_owned();

    (num, color)
}

fn parse_rules(input: &str) -> Rules {
    let mut rules = Rules {
        contained_by: HashMap::new(),
        contains: HashMap::new(),
    };

    for line in input.lines() {
        let mut split = line.split(" bags contain ");
        let container = split.next().expect("Invalid rule container");
        let contained = split.next().expect("Invalid rule contained");

        if !split.next().is_none() {
            panic!("Invalid rule split");
        }

        rules.contained_by.entry(container.to_owned()).or_default();

        let contained = match contained {
            "no other bags." => Vec::new(),
            _ => contained.split(", ").map(parse_contained_item).collect(),
        };

        for (_, v) in contained.iter() {
            let inv = rules.contained_by.entry(v.clone()).or_default();
            inv.push(container.to_owned());
        }

        rules.contains.insert(container.to_owned(), contained);
    }

    rules
}

fn dfs_containers(rules: &Rules, key: &str, containers: &mut HashSet<String>) {
    for container in rules.contained_by.get(key).unwrap() {
        containers.insert(container.clone());
        dfs_containers(&rules, &container, containers);
    }
}

fn dfs_contained_total(rules: &Rules, key: &str) -> usize {
    let contained = rules.contains.get(key).unwrap();

    let mut total = 1usize;
    for (count, color) in contained {
        total += count * dfs_contained_total(&rules, &color);
    }
    total
}

fn main() {
    let input = get_input(7);
    let rules = parse_rules(&input);

    let my_bag = "shiny gold";

    let mut containers = HashSet::<String>::new();
    dfs_containers(&rules, my_bag, &mut containers);
    dbg!(containers.len());

    dbg!(dfs_contained_total(&rules, my_bag) - 1);
}
