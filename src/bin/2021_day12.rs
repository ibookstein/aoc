use aoc::aoc_input::get_input;
use bimap::BiHashMap;
use petgraph::graph::{NodeIndex, UnGraph};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
struct CaveGraph {
    g: UnGraph<(), ()>,
    lookup: BiHashMap<String, NodeIndex>,
}

#[derive(Debug, Clone)]
struct VisitedState {
    small_visited: HashSet<NodeIndex>,
    allow_single_small_twice: bool,
    visited_twice: bool,
}

impl VisitedState {
    fn new(allow_single_small_twice: bool) -> Self {
        Self {
            small_visited: HashSet::new(),
            allow_single_small_twice,
            visited_twice: false,
        }
    }

    fn try_visit_small(&mut self, idx: NodeIndex) -> bool {
        if self.small_visited.insert(idx) {
            true
        } else if self.allow_single_small_twice && !self.visited_twice {
            self.visited_twice = true;
            true
        } else {
            false
        }
    }
}

impl CaveGraph {
    fn dfs_recurse(&self, node: NodeIndex, visited: VisitedState) -> usize {
        let neighbors: Vec<_> = self
            .g
            .neighbors(node)
            .map(|n| (self.lookup.get_by_right(&n).unwrap(), n))
            .collect();

        let mut total = 0usize;
        for (label, idx) in neighbors.iter().copied() {
            match label.as_str() {
                "start" => {}
                "end" => total += 1,
                s => {
                    let c = s.chars().next().unwrap();
                    let big = c.is_ascii_uppercase();
                    assert!(big || c.is_ascii_lowercase());
                    let mut cur_visited = visited.clone();
                    if big || cur_visited.try_visit_small(idx) {
                        total += self.dfs_recurse(idx, cur_visited);
                    }
                }
            }
        }
        total
    }

    fn count_distinct_paths(&self, allow_single_small_twice: bool) -> usize {
        let start_node = *self.lookup.get_by_left("start").unwrap();
        let visited = VisitedState::new(allow_single_small_twice);
        self.dfs_recurse(start_node, visited)
    }
}

impl FromStr for CaveGraph {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = UnGraph::new_undirected();
        let mut lookup = BiHashMap::<String, NodeIndex>::new();
        for line in s.lines() {
            let mut split = line.split('-');
            let lhs = split.next().ok_or("Missing LHS")?.to_string();
            let rhs = split.next().ok_or("Missing RHS")?.to_string();
            if split.next().is_some() {
                return Err("Trailing garbage");
            }

            let mut insert_fn = |s: String| match lookup.get_by_left(&s) {
                Some(node) => *node,
                None => {
                    let node = g.add_node(());
                    lookup.insert(s, node);
                    node
                }
            };
            let lhs_node = insert_fn(lhs);
            let rhs_node = insert_fn(rhs);
            g.add_edge(lhs_node, rhs_node, ());
        }
        Ok(Self { g, lookup })
    }
}

fn main() {
    let input = get_input(2021, 12);
    let g: CaveGraph = input.parse().unwrap();
    println!(
        "Distinct paths (visiting small caves allowed at most once): {}",
        g.count_distinct_paths(false)
    );
    println!(
        "Distinct paths (visiting single small cave twice allowed): {}",
        g.count_distinct_paths(true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    const GRAPH: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

    #[test]
    fn test_count_distinct_paths_1() {
        let g: CaveGraph = GRAPH.parse().unwrap();
        assert_eq!(g.count_distinct_paths(false), 10);
    }

    #[test]
    fn test_count_distinct_paths_2() {
        let g: CaveGraph = GRAPH.parse().unwrap();
        assert_eq!(g.count_distinct_paths(true), 36);
    }
}
