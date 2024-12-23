use ahash::{AHashMap, AHashSet};
use petgraph::prelude::*;
use std::num::ParseIntError;

pub fn parse(input: &str) -> Result<UnGraph<String, ()>, ParseIntError> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes: AHashSet<String> = AHashSet::new();
    let mut node_indices: AHashMap<String, NodeIndex> = AHashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        
        // Для первой ноды
        let n1 = if let Some(&idx) = node_indices.get(a) {
            idx
        } else {
            let idx = graph.add_node(a.to_string());
            nodes.insert(a.to_string());
            node_indices.insert(a.to_string(), idx);
            idx
        };

        // Для второй ноды
        let n2 = if let Some(&idx) = node_indices.get(b) {
            idx
        } else {
            let idx = graph.add_node(b.to_string());
            nodes.insert(b.to_string());
            node_indices.insert(b.to_string(), idx);
            idx
        };

        graph.add_edge(n1, n2, ());
    });
    
    Ok(graph)
}
