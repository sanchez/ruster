use std::fmt;
use std::ops::Deref;

use super::*;

/// Represents a node in the graph containing a value of type T.
///
/// Each node has a unique identifier and stores its value in a heap-allocated Box
/// to ensure consistent memory layout regardless of the size of T.
///
/// # Type Parameters
///
/// * `T` - The type of value stored in the node
pub struct GraphNode<T> {
    pub(crate) id: GraphNodeId,
    value: Box<T>,
}

impl<T: fmt::Debug> fmt::Debug for GraphNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphNode")
            .field("id", &self.id)
            .field("value", &self.value)
            .finish()
    }
}

impl<T: PartialEq> PartialEq for GraphNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

impl<T> GraphNode<T> {
    /// Creates a new graph node with the given ID and value.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for this node
    /// * `value` - The value to store in this node
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dater::{GraphNode, GraphNodeId};
    /// let id = GraphNodeId::new(0);
    /// let node = GraphNode::new(id, "example value");
    /// assert_eq!(*node, "example value");
    /// ```
    pub fn new(id: GraphNodeId, value: T) -> Self {
        GraphNode {
            id,
            value: Box::new(value),
        }
    }

    /// Returns the unique identifier of this node.
    ///
    /// This method is useful when you need to reference this node
    /// in graph operations like connecting nodes with edges.
    pub fn id(&self) -> GraphNodeId {
        self.id
    }
}

/// Implements Deref to allow direct access to the node's value.
///
/// This implementation makes it convenient to access the stored value
/// without explicitly dereferencing the Box.
impl<T> Deref for GraphNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation_and_access() {
        let id = GraphNodeId::new(42);
        let value = "test value";
        let node = GraphNode::new(id, value);

        assert_eq!(node.id(), id, "Node should have the assigned ID");
        assert_eq!(*node, value, "Node should contain the assigned value");
    }

    #[test]
    fn test_node_deref() {
        let node = GraphNode::new(GraphNodeId::new(0), String::from("test"));
        assert_eq!(
            node.len(),
            4,
            "Should be able to call String methods directly"
        );
    }
}
