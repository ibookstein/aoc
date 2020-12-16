use std::collections::HashMap;
use std::ops::RangeInclusive;

use aoc2020::aoc_input::get_input;

type PropRange = RangeInclusive<usize>;
type PropMap = HashMap<String, Vec<PropRange>>;

#[derive(Debug, Clone)]
struct Ticket {
    fields: Vec<usize>,
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
    for ticket in &info.tickets {
        for field in &ticket.fields {
            if flat_props.iter().copied().all(|p| !p.contains(&field)) {
                error_rate += field;
            } else {
                valid_tickets.push(ticket);
            }
        }
    }
    dbg!(error_rate);
}
