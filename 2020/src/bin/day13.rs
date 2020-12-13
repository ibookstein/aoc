use aoc2020::aoc_input::get_input;
use num_bigint::{BigInt, Sign, ToBigInt};
use num_integer::Integer;
use num_traits::One;

fn parse_input(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();
    let min_depart: usize = lines.next().unwrap().parse().unwrap();
    let buses = lines.next().unwrap().split(',');
    let bus_ids: Vec<Option<usize>> = buses.map(|s| s.parse().ok()).collect();
    (min_depart, bus_ids)
}

fn part1(min_depart: usize, bus_ids: &[Option<usize>]) {
    let (bus_id, bus_depart) = bus_ids
        .iter()
        .copied()
        .filter_map(std::convert::identity)
        .map(|id| (id, min_depart.next_multiple_of(&id)))
        .min_by_key(|(_, depart)| *depart)
        .unwrap();
    dbg!(bus_id * (bus_depart - min_depart));
}

fn part2(bus_ids: &[Option<usize>]) {
    let mut constraints = Vec::with_capacity(bus_ids.len());
    for (i, id) in bus_ids.iter().enumerate() {
        if let Some(id) = id {
            constraints.push((i.to_bigint().unwrap(), id.to_bigint().unwrap()));
        }
    }
    let constraints = constraints;
    let (mut rem1, mut mod1) = constraints[0].clone();
    for (rem2, mod2) in constraints[1..].iter() {
        let egcd = BigInt::extended_gcd(&mod1, &mod2);
        assert!(egcd.gcd.is_one());

        let new_mod = mod1.clone() * mod2;
        rem1 = (rem2 * egcd.x * mod1 + rem1 * egcd.y * mod2) % new_mod.clone();
        if rem1.sign() == Sign::Minus {
            rem1 += new_mod.clone();
        }
        mod1 = new_mod;
    }
    dbg!((mod1 - rem1).to_string());
}

fn main() {
    let input = get_input(13);
    let (min_depart, bus_ids) = parse_input(&input);
    part1(min_depart, &bus_ids);
    part2(&bus_ids);
}
