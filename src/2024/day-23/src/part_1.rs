use ahash::AHashSet;
use itertools::Itertools as _;
use petgraph::graph::UnGraph;
use std::time::{Duration, Instant};

pub fn solve(graph: &UnGraph<String, ()>) -> (usize, Duration) {
    let time = Instant::now();

    let mut visited = AHashSet::new();
    for n1 in graph.node_indices() {
        for n2 in graph.neighbors(n1) {
            if n2 == n1 {
                continue;
            }
            for n3 in graph.neighbors(n2) {
                if !graph.neighbors(n3).contains(&n1) {
                    continue;
                }

                let n1_name = graph[n1].clone();
                let n2_name = graph[n2].clone();
                let n3_name = graph[n3].clone();

                let mut key = [n1_name.clone(), n2_name.clone(), n3_name.clone()];
                key.sort_unstable();

                if visited.contains(&key) {
                    continue;
                }

                if [&n1_name, &n2_name, &n3_name]
                    .iter()
                    .any(|name| name.starts_with('t'))
                {
                    visited.insert(key);
                }
            }
        }
    }

    (visited.len(), time.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::{parser, part_1};

    #[test]
    fn part_1() {
        let input = include_str!("../datasets/debug.txt");
        let graph = parser::parse(input).unwrap();
        assert_eq!(part_1::solve(&graph).0, 7);
    }

    #[test]
    fn part_1_release() {
        let input = include_str!("../datasets/release.txt");
        let graph = parser::parse(input).unwrap();
        assert_eq!(part_1::solve(&graph).0, 1163);
    }
}
