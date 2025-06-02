use std::collections::HashMap;

use super::*;

impl<T, E> Graph<T, E> {
    pub fn node(&self, id: GraphNodeId) -> Option<&GraphNode<T>> {
        self.nodes.get(&id)
    }

    pub fn edge(&self, id: GraphEdgeId) -> Option<&GraphEdge<E>> {
        self.edges.get(&id)
    }
}
