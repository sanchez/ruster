use super::*;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GraphNodeId(usize);

impl GraphNodeId {
    pub fn new(id: usize) -> Self {
        GraphNodeId(id)
    }
}
