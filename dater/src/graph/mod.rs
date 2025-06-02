use std::collections::HashMap;

mod node;
use node::*;

mod edge;
use edge::*;

mod node_id;
use node_id::*;

mod edge_id;
use edge_id::*;

mod constructor;
mod counts;
mod getter;
mod insert;

impl<T, E> Graph<T, E> {
    fn create_node_id(&mut self) -> GraphNodeId {
        let id = GraphNodeId::new(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    fn create_edge_id(&mut self) -> GraphEdgeId {
        let id = GraphEdgeId::new(self.next_edge_id);
        self.next_edge_id += 1;
        id
    }
}

pub struct Graph<T, E = ()> {
    nodes: HashMap<GraphNodeId, GraphNode<T>>,
    edges: HashMap<GraphEdgeId, GraphEdge<E>>,

    next_node_id: usize,
    next_edge_id: usize,
}

impl<T, E> Graph<T, E> {}
