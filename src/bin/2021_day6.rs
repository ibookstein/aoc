use aoc::aoc_input::get_input;

fn main() {
    let input = get_input(2021, 6);

    let mut state = [0u64; 9];
    for timer_str in input.trim().split(',') {
        state[timer_str.parse::<usize>().unwrap() as usize] += 1;
    }

    for i in 1..=256 {
        state[(6 + i) % state.len()] += state[(8 + i) % state.len()];

        match i {
            80 | 256 => {
                let count: u64 = state.iter().sum();
                println!("Lanternfish after {} days: {}", i, count);
            }
            _ => (),
        };
    }
}
