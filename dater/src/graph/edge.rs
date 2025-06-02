use std::ops::Deref;

use super::*;

pub struct GraphEdge<E> {
    id: GraphEdgeId,
    from: GraphNodeId,
    to: GraphNodeId,
    value: Box<E>,
}

impl<E> GraphEdge<E> {
    pub fn new(id: GraphEdgeId, from: GraphNodeId, to: GraphNodeId, value: E) -> Self {
        GraphEdge {
            id,
            from,
            to,
            value: Box::new(value),
        }
    }

    pub fn id(&self) -> GraphEdgeId {
        self.id
    }

    pub fn from(&self) -> GraphNodeId {
        self.from
    }

    pub fn to(&self) -> GraphNodeId {
        self.to
    }
}

impl<T> Deref for GraphEdge<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
