use aoc2020::aoc_input::get_input;

const MODULO: u64 = 20201227;
const E: u64 = 7;

fn brute(pk: u64) -> u64 {
    let mut r = 1u64;
    let mut i = 0u64;
    loop {
        if r == pk {
            break i;
        }
        r = (r * E) % MODULO;
        i += 1;
    }
}

fn modexp(num: u64, exp: u64) -> u64 {
    let mut res = 1u64;
    for _ in 0..exp {
        res = (res * num) % MODULO;
    }
    res
}

fn main() {
    let input = get_input(25);
    let nums: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let pub1 = nums[0];
    let pub2 = nums[1];

    let pri1 = brute(pub1);
    dbg!(pri1);
    let enc = modexp(pub2, pri1);
    dbg!(enc);
}
