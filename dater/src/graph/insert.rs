use super::*;

impl<T, E> Graph<T, E> {
    /// Inserts a new node with the given value into the graph.
    ///
    /// This method creates a new node with a unique ID and the provided value,
    /// and adds it to the graph.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the new node
    ///
    /// # Returns
    ///
    /// * `Some(GraphNodeId)` - The ID of the newly created node
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, ()> = Graph::new();
    /// let node_id = graph.insert(42).unwrap();
    /// assert_eq!(**graph.node(node_id).unwrap(), 42);
    /// ```
    pub fn insert(&mut self, value: T) -> Option<GraphNodeId> {
        let id = self.create_node_id();
        let node = GraphNode::new(id, value);
        self.nodes.insert(id, node);

        Some(id)
    }

    /// Creates an edge connecting two nodes with an associated value.
    ///
    /// This method creates a directed edge from the source node to the destination node,
    /// with the provided value.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the source node
    /// * `value` - The value to store in the edge
    /// * `to` - The ID of the destination node
    ///
    /// # Returns
    ///
    /// * `Some(GraphEdgeId)` - The ID of the newly created edge
    /// * `None` - If either node doesn't exist or if attempting to create a self-loop
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::Graph;
    /// let mut graph: Graph<i32, &str> = Graph::new();
    /// let node1 = graph.insert(1).unwrap();
    /// let node2 = graph.insert(2).unwrap();
    /// let edge = graph.connect(node1, "connects to", node2);
    /// assert!(edge.is_some());
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_node() {
        let mut graph: Graph<i32, ()> = Graph::new();
        let value = 42;

        let node_id = graph.insert(value).unwrap();
        assert_eq!(graph.node_count(), 1);

        let node = graph.node(node_id).unwrap();
        assert_eq!(**node, value);
    }

    #[test]
    fn test_connect_nodes() {
        let mut graph = Graph::new();
        let node1 = graph.insert(1).unwrap();
        let node2 = graph.insert(2).unwrap();

        let edge_id = graph.connect(node1, "test", node2).unwrap();
        assert_eq!(graph.edge_count(), 1);

        let edge = graph.edge(edge_id).unwrap();
        assert_eq!(edge.from(), node1);
        assert_eq!(edge.to(), node2);
        assert_eq!(**edge, "test");
    }

    #[test]
    fn test_connect_validation() {
        let mut graph: Graph<i32, ()> = Graph::new();
        let node = graph.insert(1).unwrap();
        let invalid_node = GraphNodeId::new(999);

        // Test self-loop prevention
        assert!(
            graph.connect(node, (), node).is_none(),
            "Should not allow self-loops"
        );

        // Test non-existent source node
        assert!(
            graph.connect(invalid_node, (), node).is_none(),
            "Should not allow edges from non-existent nodes"
        );

        // Test non-existent destination node
        assert!(
            graph.connect(node, (), invalid_node).is_none(),
            "Should not allow edges to non-existent nodes"
        );
    }
}
