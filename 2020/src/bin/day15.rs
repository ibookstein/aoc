use aoc2020::aoc_input::get_input;
use std::collections::HashMap;
use std::iter::Iterator;

struct SpokenNumbers {
    start_nums: Vec<usize>,
    last_spoken: HashMap<usize, usize>,
    prev: Option<usize>,
    idx: usize,
}

impl SpokenNumbers {
    fn new(start_nums: Vec<usize>) -> SpokenNumbers {
        SpokenNumbers {
            start_nums: start_nums,
            last_spoken: HashMap::new(),
            prev: None,
            idx: 0,
        }
    }
}

impl Iterator for SpokenNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.start_nums.get(self.idx) {
            Some(n) => *n,
            None => {
                let prev = self.prev.unwrap();
                match self.last_spoken.get(&prev) {
                    None => 0,
                    Some(idx) => self.idx - idx,
                }
            }
        };

        if let Some(n) = self.prev {
            self.last_spoken.insert(n, self.idx);
        };
        self.prev = Some(ret);
        self.idx += 1;
        Some(ret)
    }
}

fn main() {
    let input = get_input(15);
    let line = input.lines().next().unwrap();
    let start_nums: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

    dbg!(SpokenNumbers::new(start_nums.clone())
        .nth(2020 - 1)
        .unwrap());
    dbg!(SpokenNumbers::new(start_nums.clone())
        .nth(30000000 - 1)
        .unwrap());
}
