use aoc2020::aoc_input::get_input;

fn main() {
    let input = get_input(9);
    let nums: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    dbg!(nums.len());

    const WINDOW: usize = 25;
    let mut rulebreak = None;

    'outer1: for i in WINDOW..nums.len() {
        let num = nums[i];

        for j in i - WINDOW..i {
            for k in j + 1..i {
                let left = nums[j];
                let right = nums[k];

                if left == right {
                    continue;
                }

                if left + right == num {
                    continue 'outer1;
                }
            }
        }

        rulebreak = Some(dbg!((i, num)));
        break;
    }

    let (invalid_num_idx, invalid_num) = rulebreak.unwrap();
    'outer2: for i in 0..invalid_num_idx - 1 {
        let ni = nums[i];
        let mut sum = ni;
        let mut min = ni;
        let mut max = ni;

        for j in i + 1..invalid_num_idx {
            let nj = nums[j];
            sum += nj;
            min = min.min(nj);
            max = max.max(nj);

            if sum == invalid_num {
                dbg!(min + max);
                break 'outer2;
            } else if sum > invalid_num {
                break;
            }
        }
    }
}
