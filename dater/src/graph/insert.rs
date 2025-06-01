use std::collections::HashMap;

use super::*;

impl<T, E> Graph<T, E> {
    pub fn insert(&mut self, value: T) -> Option<&GraphNode<T>> {
        let id = self.create_node_id();
        let node = GraphNode::new(id, value);
        self.nodes.insert(id, node);
        self.nodes.get(&id)
    }

    pub fn connect(&mut self, from: GraphId, to: GraphId, value: E) -> Option<&GraphEdge<E>> {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            let edge_id = self.create_edge_id();
            let edge = GraphEdge {
                id: edge_id,
                from,
                to,
                value: Box::new(value),
            };
            self.edges.insert(edge_id, edge);
            self.edges.get(&edge_id)
        } else {
            None
        }
    }
}
