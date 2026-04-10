use crate::position::Position;

pub type NodeValue = bool;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<N: NodeObj> {
    pub id: u64,
    pub inner: Option<N>,
}
impl<N: NodeObj> Node<N> {
    pub fn new(id: u64, inner: Option<N>) -> Self {
        Node { id, inner }
    }
}
impl<N: NodeObj> Default for Node<N> {
    fn default() -> Self {
        Node::new(0, None)
    }
}

pub trait NodeObj {
    fn position(&self) -> Position;
    fn move_to(&mut self, position: Position);
}
