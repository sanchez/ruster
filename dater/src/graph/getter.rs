use super::*;

impl<T, E> Graph<T, E> {
    /// Retrieves a reference to a node by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the node to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(&GraphNode<T>)` - A reference to the node if it exists
    /// * `None` - If no node exists with the given ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, ()> = Graph::new();
    /// let node_id = graph.insert(42).unwrap();
    /// let node = graph.node(node_id).unwrap();
    /// assert_eq!(**node, 42);
    /// ```
    pub fn node(&self, id: GraphNodeId) -> Option<&GraphNode<T>> {
        self.nodes.get(&id)
    }

    /// Retrieves a reference to an edge by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the edge to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(&GraphEdge<E>)` - A reference to the edge if it exists
    /// * `None` - If no edge exists with the given ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, f64> = Graph::new();
    /// let n1 = graph.insert(1).unwrap();
    /// let n2 = graph.insert(2).unwrap();
    /// let edge_id = graph.connect(n1, 42.0, n2).unwrap();
    /// let edge = graph.edge(edge_id).unwrap();
    /// assert_eq!(**edge, 42.0);
    /// ```
    pub fn edge(&self, id: GraphEdgeId) -> Option<&GraphEdge<E>> {
        self.edges.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_node() {
        let mut graph: Graph<String, ()> = Graph::new();
        let value = String::from("test");
        let node_id = graph.insert(value.clone()).unwrap();

        let node = graph.node(node_id).unwrap();
        assert_eq!(**node, value);

        let invalid_id = GraphNodeId::new(999);
        assert!(graph.node(invalid_id).is_none());
    }

    #[test]
    fn test_get_edge() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.insert(1).unwrap();
        let n2 = graph.insert(2).unwrap();
        let value = 42.0_f64;

        let edge_id = graph.connect(n1, value, n2).unwrap();
        let edge = graph.edge(edge_id).unwrap();
        assert_eq!(**edge, value);

        let invalid_id = GraphEdgeId::new(999);
        assert!(graph.edge(invalid_id).is_none());
    }
}
