use std::collections::HashMap;

use super::*;

impl<T, E> Graph<T, E> {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
