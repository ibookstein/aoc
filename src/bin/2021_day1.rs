use aoc::aoc_input::get_input;

fn main() {
    let input = get_input(2021, 1);
    let entries: Vec<_> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

    let increases1 = entries.windows(2).filter(|w| w[1] > w[0]).count();
    println!("Part 1 increasing measurements: {}", increases1);

    let increases2 = entries.windows(4).filter(|w| w[3] > w[0]).count();
    println!("Part 2 increasing measurements: {}", increases2);
}
