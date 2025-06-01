use std::collections::HashMap;

mod node;
use node::*;

mod constructor;
mod getter;
mod insert;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GraphId {
    id: usize,
}

impl<T, E> Graph<T, E> {
    fn create_node_id(&mut self) -> GraphId {
        let id = GraphId {
            id: self.next_node_id,
        };
        self.next_node_id += 1;
        id
    }

    fn create_edge_id(&mut self) -> GraphId {
        let id = GraphId {
            id: self.next_edge_id,
        };
        self.next_edge_id += 1;
        id
    }
}

pub struct GraphEdge<E> {
    id: GraphId,
    from: GraphId,
    to: GraphId,
    value: Box<E>,
}

pub struct Graph<T, E = ()> {
    nodes: HashMap<GraphId, GraphNode<T>>,
    edges: HashMap<GraphId, GraphEdge<E>>,

    next_node_id: usize,
    next_edge_id: usize,
}

impl<T, E> Graph<T, E> {}
