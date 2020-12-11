use aoc2020::aoc_input::get_input;

fn jolts_ascending(input: &str) -> Vec<usize> {
    let mut jolts = vec![0usize];
    jolts.extend(input.lines().map(|s| s.parse::<usize>().unwrap()));
    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);
    jolts
}

fn arrangements(jolts: &[usize]) -> usize {
    let len = jolts.len();
    if len < 2 {
        panic!("Invalid length");
    }
    if len == 2 {
        return 1;
    }

    let pivot = len / 2;
    let delta = jolts[pivot + 1] - jolts[pivot - 1];
    let left = &jolts[..=pivot];
    let right = &jolts[pivot..];

    let mut result = arrangements(left) * arrangements(right);
    if delta <= 3 {
        let mut without_pivot = Vec::with_capacity(len - 1);
        without_pivot.extend_from_slice(&jolts[..pivot]);
        without_pivot.extend_from_slice(&jolts[pivot + 1..]);
        result += arrangements(&without_pivot);
    };

    result
}

fn main() {
    let input = get_input(10);
    let jolts = jolts_ascending(&input);

    let mut deltas_hist = [0usize; 4];
    for w in jolts.windows(2) {
        let delta = w[1] - w[0];
        assert!(delta < deltas_hist.len());
        deltas_hist[delta] += 1;
    }

    dbg!(deltas_hist[1] * deltas_hist[3]);
    dbg!(arrangements(&jolts));
}
