use std::fmt;
use std::ops::Deref;

use super::*;

/// Represents a directed edge in the graph connecting two nodes.
///
/// An edge connects a source node (from) to a destination node (to) and
/// can optionally store a value of type E (e.g., for weighted graphs).
///
/// # Type Parameters
///
/// * `E` - The type of value stored in the edge
pub struct GraphEdge<E> {
    pub(crate) id: GraphEdgeId,
    pub(crate) from: GraphNodeId,
    pub(crate) to: GraphNodeId,
    value: Box<E>,
}

impl<E: fmt::Debug> fmt::Debug for GraphEdge<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphEdge")
            .field("id", &self.id)
            .field("from", &self.from)
            .field("to", &self.to)
            .field("value", &self.value)
            .finish()
    }
}

impl<E: PartialEq> PartialEq for GraphEdge<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.from == other.from
            && self.to == other.to
            && self.value == other.value
    }
}

impl<E> GraphEdge<E> {
    /// Creates a new directed edge.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this edge
    /// * `from` - ID of the source node
    /// * `to` - ID of the destination node
    /// * `value` - Value to store in this edge
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::{GraphEdge, GraphEdgeId, GraphNodeId};
    /// let edge = GraphEdge::new(
    ///     GraphEdgeId::new(0),
    ///     GraphNodeId::new(1),
    ///     GraphNodeId::new(2),
    ///     42.0
    /// );
    /// assert_eq!(*edge, 42.0);
    /// ```
    pub fn new(id: GraphEdgeId, from: GraphNodeId, to: GraphNodeId, value: E) -> Self {
        GraphEdge {
            id,
            from,
            to,
            value: Box::new(value),
        }
    }

    /// Returns the unique identifier of this edge.
    pub fn id(&self) -> GraphEdgeId {
        self.id
    }

    /// Returns the ID of the source node.
    pub fn from(&self) -> GraphNodeId {
        self.from
    }

    /// Returns the ID of the destination node.
    pub fn to(&self) -> GraphNodeId {
        self.to
    }
}

/// Implements Deref to allow direct access to the edge's value.
///
/// This implementation makes it convenient to access the stored value
/// without explicitly dereferencing the Box.
impl<T> Deref for GraphEdge<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation_and_access() {
        let edge_id = GraphEdgeId::new(1);
        let from_id = GraphNodeId::new(2);
        let to_id = GraphNodeId::new(3);
        let value = 42.0;

        let edge = GraphEdge::new(edge_id, from_id, to_id, value);

        assert_eq!(edge.id(), edge_id, "Edge should have the assigned ID");
        assert_eq!(edge.from(), from_id, "Edge should have correct source node");
        assert_eq!(
            edge.to(),
            to_id,
            "Edge should have correct destination node"
        );
        assert_eq!(*edge, value, "Edge should contain the assigned value");
    }

    #[test]
    fn test_edge_value_types() {
        // Test with different value types
        let edge_str = GraphEdge::new(
            GraphEdgeId::new(0),
            GraphNodeId::new(0),
            GraphNodeId::new(1),
            String::from("test"),
        );
        assert_eq!(
            edge_str.len(),
            4,
            "Should be able to call String methods directly"
        );
        let edge_float = GraphEdge::new(
            GraphEdgeId::new(0),
            GraphNodeId::new(0),
            GraphNodeId::new(1),
            3.14_f64,
        );
        assert!(
            (edge_float.value.as_ref() - 3.14_f64).abs() < f64::EPSILON,
            "Should store and compare f64 values"
        );
    }
}
