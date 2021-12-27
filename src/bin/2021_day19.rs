use aoc::{
    aoc_input::get_input,
    parse::{iter_consume_exact, split_exact},
    vec::ISizeVec3,
};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rotation(usize);

impl Rotation {
    const ROTATIONS: [(&'static dyn Fn(ISizeVec3) -> ISizeVec3, usize); 24] = [
        (&|d| ISizeVec3::new([d[0], d[1], d[2]]), 0),
        (&|d| ISizeVec3::new([-d[2], -d[1], -d[0]]), 1),
        (&|d| ISizeVec3::new([-d[2], -d[0], d[1]]), 8),
        (&|d| ISizeVec3::new([-d[2], d[0], -d[1]]), 16),
        (&|d| ISizeVec3::new([-d[2], d[1], d[0]]), 23),
        (&|d| ISizeVec3::new([-d[1], -d[2], d[0]]), 21),
        (&|d| ISizeVec3::new([-d[1], -d[0], -d[2]]), 6),
        (&|d| ISizeVec3::new([-d[1], d[0], d[2]]), 17),
        (&|d| ISizeVec3::new([-d[1], d[2], -d[0]]), 2),
        (&|d| ISizeVec3::new([-d[0], -d[2], -d[1]]), 9),
        (&|d| ISizeVec3::new([-d[0], -d[1], d[2]]), 10),
        (&|d| ISizeVec3::new([-d[0], d[1], -d[2]]), 11),
        (&|d| ISizeVec3::new([-d[0], d[2], d[1]]), 12),
        (&|d| ISizeVec3::new([d[0], -d[2], d[1]]), 15),
        (&|d| ISizeVec3::new([d[0], -d[1], -d[2]]), 14),
        (&|d| ISizeVec3::new([d[0], d[2], -d[1]]), 13),
        (&|d| ISizeVec3::new([d[1], -d[2], -d[0]]), 3),
        (&|d| ISizeVec3::new([d[1], -d[0], d[2]]), 7),
        (&|d| ISizeVec3::new([d[1], d[0], -d[2]]), 18),
        (&|d| ISizeVec3::new([d[1], d[2], d[0]]), 22),
        (&|d| ISizeVec3::new([d[2], -d[1], d[0]]), 20),
        (&|d| ISizeVec3::new([d[2], -d[0], -d[1]]), 5),
        (&|d| ISizeVec3::new([d[2], d[0], d[1]]), 19),
        (&|d| ISizeVec3::new([d[2], d[1], -d[0]]), 4),
    ];

    fn new(id: usize) -> Self {
        assert!(id < Self::ROTATIONS.len());
        Self(id)
    }

    fn inverse(&self) -> Self {
        Self::new(Self::ROTATIONS[self.0].1)
    }

    fn call(&self, d: ISizeVec3) -> ISizeVec3 {
        Self::ROTATIONS[self.0].0(d)
    }

    fn all() -> impl Iterator<Item = Rotation> + Clone {
        (0..Self::ROTATIONS.len()).map(Self::new)
    }
}

fn all_deltas(beacons: &HashSet<ISizeVec3>) -> HashMap<ISizeVec3, HashSet<ISizeVec3>> {
    let mut res: HashMap<ISizeVec3, HashSet<ISizeVec3>> = HashMap::new();
    for (&b1, &b2) in iproduct!(beacons.iter(), beacons.iter()) {
        if b1 != b2 {
            res.entry(b1 - b2).or_default().insert(b1);
        }
    }
    res
}

#[derive(Debug, Clone)]
struct UnbasedScanner {
    id: usize,
    beacons: HashSet<ISizeVec3>,
    deltas: HashMap<ISizeVec3, HashSet<ISizeVec3>>,
}

impl UnbasedScanner {
    fn new(id: usize, beacons: HashSet<ISizeVec3>) -> Self {
        let deltas = all_deltas(&beacons);
        Self {
            id,
            beacons,
            deltas,
        }
    }
}

#[derive(Debug, Clone)]
struct BasedScanner {
    #[allow(dead_code)]
    id: usize,
    abs_pos: ISizeVec3,
    abs_beacon_positions: HashSet<ISizeVec3>,
    deltas: HashMap<ISizeVec3, HashSet<ISizeVec3>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DeltaIntersection {
    rotation: Rotation,
    based_delta: ISizeVec3,
    unbased_delta: ISizeVec3,
}

impl BasedScanner {
    fn new(id: usize, abs_pos: ISizeVec3, abs_beacon_positions: HashSet<ISizeVec3>) -> Self {
        let deltas = all_deltas(&abs_beacon_positions);
        Self {
            id,
            abs_pos,
            abs_beacon_positions,
            deltas,
        }
    }

    fn try_detect(&self, unbased: &UnbasedScanner) -> Option<BasedScanner> {
        // b0, ..., b11 - don't know which, in rebased scanner
        // u0, ..., u11 - don't know which, in unrebased scanner
        // s - unknown, unrebased scanner absolute position
        //
        // u_i = R(b_i - s)
        // u_i - u_j = R(b_i - b_j)
        // Calculate various (u_i - u_j), (b_i - b_j), brute-force on rotations
        //
        // s = b_i - R^-1 * u_i
        // b_i = s + R^-1 * u_i

        for (based_delta, based_beacons) in self.deltas.iter() {
            for rotation in Rotation::all() {
                let unbased_delta = rotation.call(*based_delta);
                let unbased_beacons = match unbased.deltas.get(&unbased_delta) {
                    None => continue,
                    Some(beacons) => beacons,
                };

                for (&based_beacon, &unbased_beacon) in
                    iproduct!(based_beacons.iter(), unbased_beacons.iter())
                {
                    let inv = rotation.inverse();
                    let scanner_pos = based_beacon - inv.call(unbased_beacon);

                    let abs_beacons: HashSet<ISizeVec3> = unbased
                        .beacons
                        .iter()
                        .map(|u| scanner_pos + inv.call(*u))
                        .collect();

                    if self.abs_beacon_positions.intersection(&abs_beacons).count() >= 12 {
                        return Some(BasedScanner::new(unbased.id, scanner_pos, abs_beacons));
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct DetectionQueue {
    pos_known: Vec<BasedScanner>,
    pos_unknown: VecDeque<UnbasedScanner>,
}

impl DetectionQueue {
    fn new(pos_known: Vec<BasedScanner>, pos_unknown: VecDeque<UnbasedScanner>) -> Self {
        Self {
            pos_known,
            pos_unknown,
        }
    }

    fn detect_all(&mut self) {
        while let Some(unbased) = self.pos_unknown.pop_front() {
            let based = self
                .pos_known
                .iter()
                .filter_map(|based| based.try_detect(&unbased))
                .next();
            match based {
                None => self.pos_unknown.push_back(unbased),
                Some(new_scanner) => self.pos_known.push(new_scanner),
            }
        }
    }
}

fn parse_scanner_line(line: &str) -> usize {
    const START: &str = "--- scanner ";
    const END: &str = " ---";
    assert!(line.starts_with(START) && line.ends_with(END));
    let scanner_id_str = &line[START.len()..line.len() - END.len()];
    scanner_id_str.parse().unwrap()
}

fn parse_input(input: &str) -> DetectionQueue {
    let lines: Vec<_> = input.lines().collect();

    let mut pos_known = Vec::new();
    let mut pos_unknown = VecDeque::new();
    let mut scanner_count = 0usize;
    for group in lines.split(|line| line.is_empty()) {
        let scanner_id = parse_scanner_line(group[0]);
        assert_eq!(scanner_id, scanner_count);

        let mut beacons = HashSet::new();
        for beacon_line in group[1..].iter() {
            let nums: [&str; 3] = split_exact(beacon_line, ",").unwrap();
            let nums: [isize; 3] =
                iter_consume_exact(nums.into_iter().map(|s| s.parse().unwrap())).unwrap();
            beacons.insert(ISizeVec3::new(nums));
        }

        if scanner_id == 0 {
            pos_known.push(BasedScanner::new(scanner_id, ISizeVec3::zero(), beacons));
        } else {
            pos_unknown.push_back(UnbasedScanner::new(scanner_id, beacons));
        }
        scanner_count += 1;
    }

    DetectionQueue::new(pos_known, pos_unknown)
}

fn main() {
    let input = get_input(2021, 19);
    let mut queue = parse_input(&input);
    queue.detect_all();

    let beacons: HashSet<_> = queue
        .pos_known
        .iter()
        .flat_map(|s| s.abs_beacon_positions.iter())
        .collect();
    println!("Number of beacons: {}", beacons.len());

    let largest_manhattan = queue
        .pos_known
        .iter()
        .combinations(2)
        .map(|v| (v[0].abs_pos - v[1].abs_pos).manhattan_norm())
        .max()
        .unwrap();
    println!("Largest manhattan distance: {}", largest_manhattan);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_rotations() {
        let d = ISizeVec3::new([1, 2, 3]);
        let mut rds = HashSet::new();
        for rotation in Rotation::all() {
            let inv = rotation.inverse();
            assert_eq!(inv.inverse(), rotation);
            let rd = rotation.call(d);
            assert_eq!(inv.call(rd), d);
            rds.insert(rd);
        }
        assert_eq!(rds.len(), 24);
    }

    fn get_example() -> DetectionQueue {
        parse_input(include_str!("2021_day19_example.txt"))
    }

    #[test]
    fn test_detection() {
        let queue = get_example();
        let scanner0 = &queue.pos_known[0];
        assert_eq!(scanner0.id, 0);
        let scanner1_unbased = &queue.pos_unknown[0];
        assert_eq!(scanner1_unbased.id, 1);

        let scanner1 = scanner0.try_detect(&scanner1_unbased).unwrap();
        assert_eq!(scanner1.abs_pos, ISizeVec3::new([68, -1246, -43]));
        assert!(scanner1
            .abs_beacon_positions
            .contains(&ISizeVec3::new([-618, -824, -621])));
        assert!(scanner1
            .abs_beacon_positions
            .contains(&ISizeVec3::new([-537, -823, -458])));

        let scanner4_unbased = &queue.pos_unknown[3];
        assert_eq!(scanner4_unbased.id, 4);

        let scanner4 = scanner1.try_detect(&scanner4_unbased).unwrap();
        assert_eq!(scanner4.abs_pos, ISizeVec3::new([-20, -1133, 1061]));
        assert!(scanner4
            .abs_beacon_positions
            .contains(&ISizeVec3::new([459, -707, 401])));
        assert!(scanner4
            .abs_beacon_positions
            .contains(&ISizeVec3::new([-739, -1745, 668])));

        let scanner3_unbased = &queue.pos_unknown[2];
        assert_eq!(scanner3_unbased.id, 3);
        let scanner3 = scanner1.try_detect(&scanner3_unbased).unwrap();
        assert_eq!(scanner3.abs_pos, ISizeVec3::new([-92, -2380, -20]));

        let scanner2_unbased = &queue.pos_unknown[1];
        assert_eq!(scanner2_unbased.id, 2);
        let scanner2 = scanner4.try_detect(&scanner2_unbased).unwrap();
        assert_eq!(scanner2.abs_pos, ISizeVec3::new([1105, -1205, 1229]));
    }

    #[test]
    fn test_detect_all() {
        let mut queue = get_example();
        queue.detect_all();
        assert!(queue.pos_unknown.is_empty());
        assert_eq!(queue.pos_known.len(), 5);
    }
}
