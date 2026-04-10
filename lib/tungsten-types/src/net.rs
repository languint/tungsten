use crate::{nodes::NodeValue, position::Position};

pub enum WireType {
    Input,
    Output,
    Bidirectional,
}

pub struct PortRef {
    pub node_id: u64,
    pub port_name: String,
}

pub type NetId = u64;

pub struct Net {
    pub id: NetId,
    pub value: NodeValue,
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
