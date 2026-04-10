use tungsten_types::nodes::NodeObj;

use crate::nodes::traits::gate::{GateIO, GateNode, GateValue};

#[derive(Debug, Clone)]
pub struct AndGate {
    position: tungsten_types::position::Position,
    io: GateIO,
}

impl AndGate {
    pub fn new(position: tungsten_types::position::Position) -> Self {
        AndGate {
            position,
            io: GateIO::new(
                vec![GateValue::new("A", false), GateValue::new("B", false)],
                vec![GateValue::new("C", false)],
            ),
        }
    }
}

impl NodeObj for AndGate {
    fn position(&self) -> tungsten_types::position::Position {
        self.position
    }

    fn move_to(&mut self, position: tungsten_types::position::Position) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AndGateError {
    #[error("Missing input: {0}")]
    MissingInput(String),
}

impl GateNode<AndGateError> for AndGate {
    fn process(&mut self) -> Result<(), AndGateError> {
        let a = self
            .io
            .get_input("A")
            .ok_or(AndGateError::MissingInput("A".to_string()))?
            .value;
        let b = self
            .io
            .get_input("B")
            .ok_or(AndGateError::MissingInput("B".to_string()))?
            .value;

        self.io.get_output_mut("C").unwrap().set_value(a && b);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tungsten_types::position::Position;

    use super::*;

    #[test]
    fn process() {
        let mut gate = AndGate::new(Position::ZERO);

        let truth_table: [(bool, bool, bool); 4] = [
            (false, false, false),
            (false, true, false),
            (true, false, false),
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
