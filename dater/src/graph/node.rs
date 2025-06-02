use std::ops::Deref;

use super::*;

pub struct GraphNode<T> {
    id: GraphNodeId,
    value: Box<T>,
}

impl<T> GraphNode<T> {
    pub fn new(id: GraphNodeId, value: T) -> Self {
        GraphNode {
            id,
            value: Box::new(value),
        }
    }

    pub fn id(&self) -> GraphNodeId {
        self.id
    }
}

impl<T> Deref for GraphNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
