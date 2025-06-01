use std::collections::HashMap;

use super::Graph;

impl<T, E> Default for Graph<T, E> {
    fn default() -> Self {
        Graph::new()
    }
}

impl<T, E> Graph<T, E> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_node_id: 0,
            next_edge_id: 0,
        }
    }
}
