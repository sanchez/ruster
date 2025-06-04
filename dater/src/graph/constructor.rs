use std::collections::HashMap;

use super::Graph;

/// Provides a default empty graph implementation.
///
/// This implementation allows Graph to be used in contexts requiring Default.
impl<T, E> Default for Graph<T, E> {
    fn default() -> Self {
        Graph::new()
    }
}

impl<T, E> Graph<T, E> {
    /// Creates a new empty graph.
    ///
    /// The graph is initialized with no nodes or edges, ready to store values
    /// of type T for nodes and E for edges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// // Create a graph with string nodes and f64 edges
    /// let graph: Graph<String, f64> = Graph::new();
    /// assert_eq!(graph.node_count(), 0);
    /// assert_eq!(graph.edge_count(), 0);
    /// ```
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_node_id: 0,
            next_edge_id: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        let graph: Graph<i32, ()> = Graph::new();
        assert_eq!(graph.node_count(), 0, "New graph should have no nodes");
        assert_eq!(graph.edge_count(), 0, "New graph should have no edges");
    }

    #[test]
    fn test_default_creates_empty_graph() {
        let graph: Graph<i32, ()> = Graph::default();
        assert_eq!(graph.node_count(), 0, "Default graph should have no nodes");
        assert_eq!(graph.edge_count(), 0, "Default graph should have no edges");
    }
}
