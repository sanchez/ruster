#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct GraphNodeId(usize);

impl GraphNodeId {
    pub fn new(id: usize) -> Self {
        GraphNodeId(id)
    }
}
