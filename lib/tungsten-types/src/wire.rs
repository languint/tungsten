use crate::position::Position;

pub enum WireType {
    Input,
    Output,
    Bidirectional,
}

/// Graphical representation of a wire direction, including diagonals
pub enum WireDirection {
    North,
    South,
    East,
    West,
}

pub trait WireObj {
    fn id(&self) -> u64;
    fn wire_type(&self) -> WireType;
    fn set_wire_type(&mut self, wire_type: WireType);
}

pub struct Wire {
    pub id: u64,
    pub wire_type: WireType,
    pub wires: Vec<(Position, WireType)>,
}
