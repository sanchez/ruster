use std::collections::HashMap;

use super::*;

impl<T, E> Graph<T, E> {
    pub fn insert(&mut self, value: T) -> Option<GraphNodeId> {
        let id = self.create_node_id();
        let node = GraphNode::new(id, value);
        self.nodes.insert(id, node);

        Some(id)
    }

    pub fn connect(&mut self, from: GraphNodeId, value: E, to: GraphNodeId) -> Option<GraphEdgeId> {
        if from == to {
            return None; // Prevent self-loops
        }

        if !self.nodes.contains_key(&from) {
            return None;
        }

        if !self.nodes.contains_key(&to) {
            return None;
        }

        let edge_id = self.create_edge_id();
        let edge = GraphEdge::new(edge_id, from, to, value);
        self.edges.insert(edge_id, edge);
        Some(edge_id)
    }
}
