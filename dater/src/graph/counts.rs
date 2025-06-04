use super::*;

impl<T, E> Graph<T, E> {
    /// Returns the total number of nodes in the graph.
    ///
    /// This count represents all nodes currently stored in the graph,
    /// regardless of whether they are connected by edges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, ()> = Graph::new();
    /// assert_eq!(graph.node_count(), 0);
    /// graph.insert(42);
    /// assert_eq!(graph.node_count(), 1);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the total number of edges in the graph.
    ///
    /// This count represents all directed edges currently stored in the graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, ()> = Graph::new();
    /// let n1 = graph.insert(1).unwrap();
    /// let n2 = graph.insert(2).unwrap();
    /// assert_eq!(graph.edge_count(), 0);
    /// graph.connect(n1, (), n2);
    /// assert_eq!(graph.edge_count(), 1);
    /// ```
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_counting() {
        let mut graph: Graph<i32, ()> = Graph::new();
        assert_eq!(graph.node_count(), 0, "New graph should have no nodes");

        graph.insert(1);
        assert_eq!(graph.node_count(), 1, "Graph should have one node");

        graph.insert(2);
        assert_eq!(graph.node_count(), 2, "Graph should have two nodes");
    }

    #[test]
    fn test_edge_counting() {
        let mut graph: Graph<i32, ()> = Graph::new();
        assert_eq!(graph.edge_count(), 0, "New graph should have no edges");

        let n1 = graph.insert(1).unwrap();
        let n2 = graph.insert(2).unwrap();
        let n3 = graph.insert(3).unwrap();

        graph.connect(n1, (), n2);
        assert_eq!(graph.edge_count(), 1, "Graph should have one edge");

        graph.connect(n2, (), n3);
        assert_eq!(graph.edge_count(), 2, "Graph should have two edges");

        // Adding an invalid edge shouldn't change the count
        graph.connect(n1, (), GraphNodeId::new(999));
        assert_eq!(
            graph.edge_count(),
            2,
            "Invalid edge should not affect count"
        );
    }
}
