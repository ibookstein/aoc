use aoc2020::aoc_input::get_input;
use std::collections::HashSet;

fn main() {
    let input = get_input(1);
    let entries: HashSet<_> = input.lines().map(|n| n.parse::<u64>().unwrap()).collect();

    for entry in entries.iter().copied() {
        let other = 2020 - entry;
        if entries.contains(&other) {
            println!("{} * {} = {}", entry, other, entry * other);
            break;
        }
    }

    'outer: for e1 in entries.iter().copied() {
        for e2 in entries.iter().copied() {
            let s = e1 + e2;
            if s > 2020 {
                continue;
            }
            let e3 = 2020 - s;
            if entries.contains(&e3) {
                println!("{} * {} * {} = {}", e1, e2, e3, e1 * e2 * e3);
                break 'outer;
            }
        }
    }
}
