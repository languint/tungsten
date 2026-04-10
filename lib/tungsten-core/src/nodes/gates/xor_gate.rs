use tungsten_types::nodes::NodeObj;

use crate::nodes::traits::gate::{GateIO, GateNode, GateValue};

#[derive(Debug, Clone)]
pub struct XOrGate {
    position: tungsten_types::position::Position,
    io: GateIO,
}

impl XOrGate {
    pub fn new(position: tungsten_types::position::Position) -> Self {
        XOrGate {
            position,
            io: GateIO::new(
                vec![GateValue::new("A", false), GateValue::new("B", false)],
                vec![GateValue::new("C", false)],
            ),
        }
    }
}

impl NodeObj for XOrGate {
    fn position(&self) -> tungsten_types::position::Position {
        self.position
    }

    fn move_to(&mut self, position: tungsten_types::position::Position) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum XOrGateError {
    #[error("Missing input: {0}")]
    MissingInput(String),
}

impl GateNode<XOrGateError> for XOrGate {
    fn process(&mut self) -> Result<(), XOrGateError> {
        let a = self
            .io
            .get_input("A")
            .ok_or(XOrGateError::MissingInput("A".to_string()))?
            .value;
        let b = self
            .io
            .get_input("B")
            .ok_or(XOrGateError::MissingInput("B".to_string()))?
            .value;

        self.io.get_output_mut("C").unwrap().set_value(a ^ b);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tungsten_types::position::Position;

    use super::*;

    #[test]
    fn process() {
        let mut gate = XOrGate::new(Position::ZERO);

        let truth_table: [(bool, bool, bool); 4] = [
            (false, true, true),
            (true, false, true),
            (true, true, false),
            (false, false, false),
        ];

        for (a, b, expected) in truth_table {
            gate.io.get_input_mut("A").unwrap().set_value(a);
            gate.io.get_input_mut("B").unwrap().set_value(b);

            gate.process().expect("This should succeed");

            assert_eq!(gate.io.get_output("C").unwrap().value, expected);
        }
    }
}
