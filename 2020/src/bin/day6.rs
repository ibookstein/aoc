use aoc2020::aoc_input::get_input;

fn main() {
    let input = get_input(6);
    let lines: Vec<_> = input.lines().collect();

    let mut any_total_count = 0usize;
    let mut all_total_count = 0usize;
    for group in lines.split(|line| line.is_empty()) {
        let mut answers_any = [false; 26];
        let mut answers_all = [true; 26];

        for answers in group {
            let mut answers_cur = [false; 26];
            for ch in answers.chars() {
                let idx = ch as usize - 'a' as usize;
                answers_cur[idx] = true;
                answers_any[idx] = true;
            }

            for i in 0..answers_all.len() {
                answers_all[i] &= answers_cur[i];
            }
        }

        any_total_count += answers_any.iter().filter(|b| **b).count();
        all_total_count += answers_all.iter().filter(|b| **b).count();
    }

    dbg!(any_total_count);
    dbg!(all_total_count);
}
