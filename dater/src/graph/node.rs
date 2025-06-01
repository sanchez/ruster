use std::ops::Deref;

use super::GraphId;

pub struct GraphNode<T> {
    id: GraphId,
    value: Box<T>,
}

impl<T> GraphNode<T> {
    pub fn new(id: GraphId, value: T) -> Self {
        GraphNode {
            id,
            value: Box::new(value),
        }
    }
}

impl<T> Deref for GraphNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
