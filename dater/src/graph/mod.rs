use std::collections::HashMap;

pub struct GraphId {
    id: usize,
}

pub struct GraphNode<T> {
    id: GraphId,
    value: Box<T>,
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
