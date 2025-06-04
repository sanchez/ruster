use std::collections::HashMap;

/// Module for graph data structure components
mod node;
pub use node::*;

mod edge;
pub use edge::*;

mod node_id;
pub use node_id::*;

mod edge_id;
pub use edge_id::*;

mod constructor;
mod counts;
mod getter;
mod insert;

/// A generic graph implementation supporting typed nodes and edges.
///
/// This implementation provides a flexible graph data structure where:
/// - Nodes can hold values of type `T`
/// - Edges can hold values of type `E` (defaults to unit type `()` for unweighted graphs)
/// - Both nodes and edges are identified by unique IDs
///
/// # Type Parameters
///
/// * `T` - The type of values stored in nodes
/// * `E` - The type of values stored in edges (defaults to `()`)
///
/// # Examples
///
/// ```rust
/// # use dater::Graph;
/// // Create a graph with i32 nodes and &str edges
/// let mut graph: Graph<i32, &str> = Graph::new();
///
/// // Add nodes and connect them
/// let node1 = graph.insert(42).unwrap();
/// let node2 = graph.insert(24).unwrap();
/// graph.connect(node1, "connects to", node2);
/// ```
pub struct Graph<T, E = ()> {
    nodes: HashMap<GraphNodeId, GraphNode<T>>,
    edges: HashMap<GraphEdgeId, GraphEdge<E>>,

    next_node_id: usize,
    next_edge_id: usize,
}

impl<T, E> Graph<T, E> {
    /// Creates a new node ID.
    ///
    /// This is an internal method used to generate unique identifiers for nodes.
    /// The ID is guaranteed to be unique within this graph instance.
    fn create_node_id(&mut self) -> GraphNodeId {
        let id = GraphNodeId::new(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    /// Creates a new edge ID.
    ///
    /// This is an internal method used to generate unique identifiers for edges.
    /// The ID is guaranteed to be unique within this graph instance.
    fn create_edge_id(&mut self) -> GraphEdgeId {
        let id = GraphEdgeId::new(self.next_edge_id);
        self.next_edge_id += 1;
        id
    }
}

// Module-level tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node_id_uniqueness() {
        let mut graph: Graph<(), ()> = Graph::new();
        let id1 = graph.create_node_id();
        let id2 = graph.create_node_id();
        assert_ne!(id1, id2, "Node IDs should be unique");
    }

    #[test]
    fn test_create_edge_id_uniqueness() {
        let mut graph: Graph<(), ()> = Graph::new();
        let id1 = graph.create_edge_id();
        let id2 = graph.create_edge_id();
        assert_ne!(id1, id2, "Edge IDs should be unique");
    }
}
