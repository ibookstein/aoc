use std::collections::HashMap;
use std::ops::RangeInclusive;

use aoc2020::aoc_input::get_input;

type PropRange = RangeInclusive<usize>;
type PropMap = HashMap<String, Vec<PropRange>>;

#[derive(Debug, Clone)]
struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn check(&self, index: usize, ranges: &[PropRange]) -> bool {
        let field = self.fields[index];
        ranges.iter().any(|r| r.contains(&field))
    }
}

#[derive(Debug)]
struct Info {
    props: PropMap,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

fn parse_ticket(s: &str) -> Ticket {
    Ticket {
        fields: s.split(',').map(|v| v.parse().unwrap()).collect(),
    }
}

fn parse_input(input: &str) -> Info {
    let lines: Vec<_> = input.lines().collect();
    let mut groups = lines.split(|line| line.is_empty());
    let props = groups.next().unwrap();
    let my_ticket = &groups.next().unwrap()[1..];
    let tickets = &groups.next().unwrap()[1..];
    assert!(groups.next().is_none());

    let mut prop_map = PropMap::new();
    for prop in props.iter().copied() {
        let mut split = prop.split(": ");
        let name = split.next().unwrap();
        let value = split.next().unwrap();
        assert!(split.next().is_none());

        let mut ranges = Vec::<PropRange>::new();
        for range_str in value.split(" or ") {
            let mut range_split = range_str.split('-');
            let min: usize = range_split.next().unwrap().parse().unwrap();
            let max: usize = range_split.next().unwrap().parse().unwrap();
            assert!(range_split.next().is_none());
            ranges.push(min..=max);
        }
        prop_map.insert(name.to_owned(), ranges);
    }

    Info {
        props: prop_map,
        my_ticket: parse_ticket(my_ticket[0]),
        tickets: tickets.iter().copied().map(parse_ticket).collect(),
    }
}

fn main() {
    let input = get_input(16);
    let info = parse_input(&input);

    let flat_props: Vec<_> = info.props.values().flatten().collect();
    let mut valid_tickets = Vec::<&Ticket>::new();
    let mut error_rate = 0usize;
    'outer: for ticket in &info.tickets {
        for field in &ticket.fields {
            if flat_props.iter().copied().all(|p| !p.contains(&field)) {
                error_rate += field;
                continue 'outer;
            }
        }
        valid_tickets.push(ticket);
    }
    dbg!(error_rate);

    let valid_tickets = valid_tickets;
    assert!(valid_tickets
        .iter()
        .copied()
        .all(|t| t.fields.len() == info.props.len()));

    let mut possible_assignments = Vec::new();
    for (name, ranges) in info.props.iter() {
        let mut cur_assignments = Vec::<usize>::new();
        for i in 0..info.props.len() {
            if !valid_tickets.iter().copied().all(|t| t.check(i, ranges)) {
                continue;
            }

            cur_assignments.push(i);
        }
        possible_assignments.push((name, cur_assignments));
    }

    let mut assignments = HashMap::new();
    while !possible_assignments.is_empty() {
        let pos = possible_assignments
            .iter()
            .position(|(_, inds)| inds.len() == 1)
            .unwrap();

        let (name, inds) = possible_assignments.remove(pos);
        let ind = inds[0];
        for (_, inds) in possible_assignments.iter_mut() {
            if let Some(pos) = inds.iter().position(|i| *i == ind) {
                inds.swap_remove(pos);
            }
        }
        assignments.insert(name, ind);
    }

    let mut product = 1usize;
    for (name, ind) in assignments.iter() {
        if name.starts_with("departure") {
            product *= info.my_ticket.fields[*ind];
        }
    }
    dbg!(product);
}
