use ahash::AHashSet;
use petgraph::graph::UnGraph;
use std::time::{Duration, Instant};
use itertools::Itertools as _;

pub fn solve(graph: &UnGraph<String, ()>) -> (String, Duration) {
    let time = Instant::now();
    let mut max_party = String::default();
    let mut seen_party = AHashSet::new();
    let mut max_size = 0;

    // для каждой вершины проверяем все возможные комбинации её соседей
    for node in graph.node_indices() {
        let mut neighbors: Vec<_> = graph.neighbors(node).collect();
        neighbors.push(node);

        // проверяем все возможные подмножества соседей
        for size in (1..=neighbors.len()).rev() {
            if size < max_size {
                break;
            }
            for combination in neighbors.iter().copied().combinations(size) {
                let mut key: Vec<_> = combination.iter().map(|&idx| graph[idx].as_str()).collect();
                key.sort();
                let key = key.join(",");

                if seen_party.contains(&key) {
                    continue;
                }
                seen_party.insert(key.clone());

                // проверяем, что все вершины соединены друг с другом
                let is_party = combination.iter().all(|&n1| {
                    combination
                        .iter()
                        .all(|&n2| n1 == n2 || graph.contains_edge(n1, n2))
                });

                if is_party {
                    if combination.len() > max_size {
                        max_size = combination.len();
                        max_party = key;
                    }
                }
            }
        }
    }

    (max_party, time.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::{parser, part_2};

    #[test]
    fn part_2() {
        let input = include_str!("../datasets/debug.txt");
        let graph = parser::parse(input).unwrap();
        assert_eq!(part_2::solve(&graph).0, "co,de,ka,ta".to_string());
    }

    #[test]
    fn part_2_release() {
        let input = include_str!("../datasets/release.txt");
        let graph = parser::parse(input).unwrap();
        assert_eq!(
            part_2::solve(&graph).0,
            "bm,bo,ee,fo,gt,hv,jv,kd,md,mu,nm,wx,xh".to_string()
        );
    }
}
