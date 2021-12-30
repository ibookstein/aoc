use aoc::{aoc_input::get_input, parse::parse_prefix_and_split};
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct ClosedInterval {
    imp: Option<(isize, isize)>,
}

#[derive(Debug, Clone, Default)]
struct ClosedIntervalDifference {
    difference: [ClosedInterval; 2],
    intersection: ClosedInterval,
}

impl ClosedInterval {
    fn new(min: isize, max: isize) -> Self {
        Self {
            imp: if min > max { None } else { Some((min, max)) },
        }
    }

    fn empty() -> Self {
        Self { imp: None }
    }

    fn is_empty(&self) -> bool {
        self.imp.is_none()
    }

    fn len(&self) -> usize {
        match self.imp {
            None => 0,
            Some((min, max)) => (max - min + 1) as usize,
        }
    }

    fn contains(&self, other: &ClosedInterval) -> bool {
        match (self.imp, other.imp) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some((self_min, self_max)), Some((other_min, other_max))) => {
                self_min <= other_min && other_max <= self_max
            }
        }
    }

    fn difference(&self, other: &ClosedInterval) -> ClosedIntervalDifference {
        match (self.imp, other.imp) {
            (None, _) => Default::default(),
            (Some(_), None) => ClosedIntervalDifference {
                difference: [*self, ClosedInterval::empty()],
                intersection: ClosedInterval::empty(),
            },
            (Some((self_min, self_max)), Some((other_min, other_max))) => {
                ClosedIntervalDifference {
                    difference: [
                        ClosedInterval::new(self_min, min(other_min - 1, self_max)),
                        ClosedInterval::new(max(self_min, other_max + 1), self_max),
                    ],
                    intersection: ClosedInterval::new(
                        max(self_min, other_min),
                        min(self_max, other_max),
                    ),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    bounds: [ClosedInterval; 3],
}

impl Cuboid {
    fn new(bounds: [ClosedInterval; 3]) -> Self {
        Self { bounds }
    }

    fn contains(&self, other: &Cuboid) -> bool {
        self.bounds
            .into_iter()
            .enumerate()
            .all(|(i, b)| b.contains(&other.bounds[i]))
    }

    fn volume(&self) -> usize {
        self.bounds.iter().map(ClosedInterval::len).product()
    }

    fn difference(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut res = Vec::with_capacity(2 * self.bounds.len());

        let mut cur_bounds = self.bounds;
        for (i, b) in self.bounds.into_iter().enumerate() {
            let diff = b.difference(&other.bounds[i]);
            if diff.intersection.is_empty() {
                return vec![*self];
            }

            for interval in diff.difference.iter().filter(|b| !b.is_empty()) {
                cur_bounds[i] = *interval;
                res.push(Cuboid::new(cur_bounds));
            }
            cur_bounds[i] = diff.intersection;
        }

        res
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    cuboid: Cuboid,
    state: bool,
}

#[derive(Debug, Clone)]
struct Reactor {
    on_cuboids: Vec<Cuboid>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            on_cuboids: Vec::new(),
        }
    }

    fn process_step(&mut self, step: &Step) {
        let mut new_state = Vec::with_capacity(self.on_cuboids.len());
        for cuboid in &self.on_cuboids {
            new_state.append(&mut cuboid.difference(&step.cuboid));
        }
        if step.state {
            new_state.push(step.cuboid);
        }

        self.on_cuboids = new_state;
    }

    fn process_steps<'a>(&mut self, steps: impl IntoIterator<Item = &'a Step>) {
        for step in steps {
            self.process_step(step);
        }
    }

    fn total_on(&self) -> usize {
        self.on_cuboids.iter().map(Cuboid::volume).sum()
    }
}

fn parse_bound(s: &str, prefix: &str) -> ClosedInterval {
    let [min_str, max_str] = parse_prefix_and_split(s, prefix, "..").unwrap();
    ClosedInterval::new(min_str.parse().unwrap(), max_str.parse().unwrap())
}

fn parse_step(line: &str) -> Step {
    let mut split = line.split(' ');
    let state = match split.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => panic!("Invalid state string"),
    };

    let cuboid_str = split.next().unwrap();
    assert!(split.next().is_none());
    let mut split = cuboid_str.split(',');
    let xbound = parse_bound(split.next().unwrap(), "x=");
    let ybound = parse_bound(split.next().unwrap(), "y=");
    let zbound = parse_bound(split.next().unwrap(), "z=");

    let cuboid = Cuboid::new([xbound, ybound, zbound]);
    Step { cuboid, state }
}

fn parse_input(input: &str) -> Vec<Step> {
    input.lines().map(parse_step).collect()
}

fn main() {
    let input = get_input(2021, 22);
    let steps = parse_input(&input);

    let part1_region = Cuboid {
        bounds: [ClosedInterval::new(-50, 50); 3],
    };
    let part1_count = steps
        .iter()
        .position(|s| !part1_region.contains(&s.cuboid))
        .unwrap();

    let mut reactor = Reactor::new();
    reactor.process_steps(steps[0..part1_count].iter());
    println!("Part 1 total cubes on: {}", reactor.total_on());

    reactor.process_steps(steps[part1_count..].iter());
    println!("Part 2 total cubes on: {}", reactor.total_on());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_bound_difference() {
        let bound = ClosedInterval::new(0, 3);
        let empty = ClosedInterval::empty();

        let res = bound.difference(&ClosedInterval::new(-2, -1));
        assert_eq!(res.difference, [empty, bound]);
        assert_eq!(res.intersection, empty);

        let res = bound.difference(&ClosedInterval::new(-2, 0));
        assert_eq!(res.difference, [empty, ClosedInterval::new(1, 3)]);
        assert_eq!(res.intersection, ClosedInterval::new(0, 0));

        let res = bound.difference(&ClosedInterval::new(0, 0));
        assert_eq!(res.difference, [empty, ClosedInterval::new(1, 3)]);
        assert_eq!(res.intersection, ClosedInterval::new(0, 0));

        let res = bound.difference(&ClosedInterval::new(0, 1));
        assert_eq!(res.difference, [empty, ClosedInterval::new(2, 3)]);
        assert_eq!(res.intersection, ClosedInterval::new(0, 1));

        let res = bound.difference(&ClosedInterval::new(1, 1));
        assert_eq!(
            res.difference,
            [ClosedInterval::new(0, 0), ClosedInterval::new(2, 3)]
        );
        assert_eq!(res.intersection, ClosedInterval::new(1, 1));

        let res = bound.difference(&ClosedInterval::new(1, 2));
        assert_eq!(
            res.difference,
            [ClosedInterval::new(0, 0), ClosedInterval::new(3, 3)]
        );
        assert_eq!(res.intersection, ClosedInterval::new(1, 2));

        let res = bound.difference(&ClosedInterval::new(1, 3));
        assert_eq!(res.difference, [ClosedInterval::new(0, 0), empty]);
        assert_eq!(res.intersection, ClosedInterval::new(1, 3));

        let res = bound.difference(&ClosedInterval::new(2, 3));
        assert_eq!(res.difference, [ClosedInterval::new(0, 1), empty]);
        assert_eq!(res.intersection, ClosedInterval::new(2, 3));

        let res = bound.difference(&ClosedInterval::new(2, 4));
        assert_eq!(res.difference, [ClosedInterval::new(0, 1), empty]);
        assert_eq!(res.intersection, ClosedInterval::new(2, 3));

        let res = bound.difference(&ClosedInterval::new(3, 3));
        assert_eq!(res.difference, [ClosedInterval::new(0, 2), empty]);
        assert_eq!(res.intersection, ClosedInterval::new(3, 3));

        let res = bound.difference(&ClosedInterval::new(4, 5));
        assert_eq!(res.difference, [bound, empty]);
        assert_eq!(res.intersection, empty);
    }

    #[test]
    fn test_cuboid_difference() {
        let cb = ClosedInterval::new(0, 2);
        let cuboid = Cuboid::new([cb; 3]);
        let ib = ClosedInterval::new(1, 1);
        let interior = Cuboid::new([ib; 3]);

        let res = cuboid.difference(&interior);
        assert_eq!(
            res,
            [
                Cuboid::new([ClosedInterval::new(0, 0), cb, cb]),
                Cuboid::new([ClosedInterval::new(2, 2), cb, cb]),
                Cuboid::new([ib, ClosedInterval::new(0, 0), cb]),
                Cuboid::new([ib, ClosedInterval::new(2, 2), cb]),
                Cuboid::new([ib, ib, ClosedInterval::new(0, 0)]),
                Cuboid::new([ib, ib, ClosedInterval::new(2, 2)]),
            ]
        );

        let foreign = Cuboid::new([ClosedInterval::new(4, 4); 3]);
        assert_eq!(cuboid.difference(&foreign), [cuboid]);

        let other = Cuboid::new([ClosedInterval::new(1, 4), cb, cb]);
        let res = cuboid.difference(&other);
        assert_eq!(res, [Cuboid::new([ClosedInterval::new(0, 0), cb, cb])]);

        let other = Cuboid::new([
            ClosedInterval::new(1, 1),
            ClosedInterval::new(-5, 5),
            ClosedInterval::new(-6, 6),
        ]);
        let res = cuboid.difference(&other);
        assert_eq!(
            res,
            [
                Cuboid::new([ClosedInterval::new(0, 0), cb, cb]),
                Cuboid::new([ClosedInterval::new(2, 2), cb, cb]),
            ]
        );

        let other = Cuboid::new([ib, ib, ClosedInterval::new(1, 2)]);
        let res = cuboid.difference(&other);
        assert_eq!(
            res,
            [
                Cuboid::new([ClosedInterval::new(0, 0), cb, cb]),
                Cuboid::new([ClosedInterval::new(2, 2), cb, cb]),
                Cuboid::new([ib, ClosedInterval::new(0, 0), cb]),
                Cuboid::new([ib, ClosedInterval::new(2, 2), cb]),
                Cuboid::new([ib, ib, ClosedInterval::new(0, 0)]),
            ]
        );
    }

    #[test]
    fn test_reactor_steps() {
        let steps = parse_input(include_str!("2021_day22_example.txt"));
        let mut reactor = Reactor::new();
        reactor.process_steps(&steps);
        assert_eq!(reactor.total_on(), 590784);
    }
}
