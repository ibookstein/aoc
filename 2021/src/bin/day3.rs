use aoc2021::aoc_input::get_input;

#[derive(Debug, Clone, Copy)]
enum Select {
    MostCommonPrefer1,
    LeastCommonPrefer0,
}

fn rating(mut nums: Vec<u32>, width: usize, sel: Select) -> u32 {
    let mut bitpos = width;
    while nums.len() > 1 {
        bitpos -= 1;

        let bitmask = 1u32 << bitpos;
        let onecount = nums.iter().filter(|&n| *n & bitmask != 0).count();
        let zerocount = nums.len() - onecount;

        let keepval = match sel {
            Select::MostCommonPrefer1 => ((onecount >= zerocount) as u32) * bitmask,
            Select::LeastCommonPrefer0 => ((onecount < zerocount) as u32) * bitmask,
        };

        nums.retain(|&n| n & bitmask == keepval);
    }

    nums[0]
}

fn main() {
    let input = get_input(3);
    let lines = input.trim().lines();
    let nums: Vec<_> = lines
        .clone()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let width = lines.clone().next().unwrap().len();
    let mut hist = vec![0u32; width];
    for &num in &nums {
        for i in 0..width {
            hist[i] += (num >> i) & 1;
        }
    }

    let half = (nums.len() / 2) as u32;
    let gamma: u32 = hist
        .iter()
        .enumerate()
        .map(|(i, &v)| ((v > half) as u32) << i)
        .sum();
    let epsilon = (!gamma) & ((1u32 << width) - 1);
    dbg!(gamma * epsilon);

    let oxygen = rating(nums.clone(), width, Select::MostCommonPrefer1);
    let co2 = rating(nums, width, Select::LeastCommonPrefer0);
    dbg!(oxygen * co2);
}
