use super::*;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GraphEdgeId(usize);

impl GraphEdgeId {
    pub fn new(id: usize) -> Self {
        GraphEdgeId(id)
    }
}
