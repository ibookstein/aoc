use aoc2020::aoc_input::get_input;

fn main() {
    let input = get_input(3);
    let lines: Vec<_> = input.lines().collect();
    const COUNT: usize = 5;
    let slopes: [(usize, usize); COUNT] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tree_counts = [0usize; COUNT];

    for scale in 1usize.. {
        let mut stop = true;
        for slope_idx in 0..COUNT {
            let slope = slopes[slope_idx];
            let tree_count = &mut tree_counts[slope_idx];

            let coord = (scale * slope.0, scale * slope.1);
            if coord.1 >= lines.len() {
                continue;
            }

            stop = false;
            let row = lines[coord.1];
            let ch = row.as_bytes()[coord.0 % row.len()];
            *tree_count += (ch == '#' as u8) as usize;
        }
        if stop {
            break;
        }
    }

    dbg!(tree_counts[1]);
    let product = tree_counts.iter().product::<usize>();
    dbg!(product);
}
