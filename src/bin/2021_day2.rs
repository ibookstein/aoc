use aoc::aoc_input::get_input;

fn main() {
    let input = get_input(2021, 2);
    let lines: Vec<_> = input.lines().collect();

    let mut horizontal1 = 0usize;
    let mut depth1 = 0usize;

    let mut horizontal2 = 0usize;
    let mut depth2 = 0usize;
    let mut aim2 = 0usize;
    for &line in &lines {
        let mut split = line.split(' ');
        let dir = split.next().expect("Invalid input");
        let amount = split.next().expect("Invalid input");
        assert!(split.next().is_none());

        let amount: usize = amount.parse().unwrap();
        match dir {
            "forward" => {
                horizontal1 += amount;
                horizontal2 += amount;
                depth2 += aim2 * amount;
            }
            "down" => {
                depth1 += amount;
                aim2 += amount;
            }
            "up" => {
                depth1 -= amount;
                aim2 -= amount;
            }
            _ => panic!("Invalid input"),
        }
    }

    dbg!(horizontal1 * depth1);
    dbg!(horizontal2 * depth2);
}
