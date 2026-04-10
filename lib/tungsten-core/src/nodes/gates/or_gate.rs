use tungsten_types::nodes::NodeObj;

use crate::nodes::traits::gate::{GateIO, GateNode, GateValue};

#[derive(Debug, Clone)]
pub struct OrGate {
    pub position: tungsten_types::position::Position,
    pub io: GateIO,
}

impl OrGate {
    pub fn new(position: tungsten_types::position::Position) -> Self {
        OrGate {
            position,
            io: GateIO::new(
                vec![GateValue::new("A", false), GateValue::new("B", false)],
                vec![GateValue::new("C", false)],
            ),
        }
    }
}

impl NodeObj for OrGate {
    fn position(&self) -> tungsten_types::position::Position {
        self.position
    }

    fn move_to(&mut self, position: tungsten_types::position::Position) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrGateError {
    #[error("Missing input: {0}")]
    MissingInput(String),
    #[error("Missing output: {0}")]
    MissingOutput(String),
}

impl GateNode<OrGateError> for OrGate {
    fn process(&mut self) -> Result<(), OrGateError> {
        let a = self
            .io
            .get_input("A")
            .ok_or(OrGateError::MissingInput("A".to_string()))?
            .value;
        let b = self
            .io
            .get_input("B")
            .ok_or(OrGateError::MissingInput("B".to_string()))?
            .value;

        self.io
            .get_output_mut("C")
            .ok_or(OrGateError::MissingOutput("C".to_string()))?
            .set_value(a || b);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tungsten_types::position::Position;

    use super::*;

    #[test]
    fn process() {
        let mut gate = OrGate::new(Position::ZERO);

        let truth_table: [(bool, bool, bool); 4] = [
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];

        for (a, b, expected) in truth_table {
            gate.io.get_input_mut("A").unwrap().set_value(a);
            gate.io.get_input_mut("B").unwrap().set_value(b);

            gate.process().expect("This should succeed");

            assert_eq!(gate.io.get_output("C").unwrap().value, expected);
        }
    }
}
