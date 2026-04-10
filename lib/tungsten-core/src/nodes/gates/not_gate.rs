use tungsten_types::nodes::NodeObj;

use crate::nodes::traits::gate::{GateIO, GateNode, GateValue};

#[derive(Debug, Clone)]
pub struct NotGate {
    pub position: tungsten_types::position::Position,
    pub io: GateIO,
}

impl NotGate {
    pub fn new(position: tungsten_types::position::Position) -> Self {
        NotGate {
            position,
            io: GateIO::new(
                vec![GateValue::new("A", false)],
                vec![GateValue::new("B", false)],
            ),
        }
    }
}

impl NodeObj for NotGate {
    fn position(&self) -> tungsten_types::position::Position {
        self.position
    }

    fn move_to(&mut self, position: tungsten_types::position::Position) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NotGateError {
    #[error("Missing input: {0}")]
    MissingInput(String),
    #[error("Missing output: {0}")]
    MissingOutput(String),
}

impl GateNode<NotGateError> for NotGate {
    fn process(&mut self) -> Result<(), NotGateError> {
        let a = self
            .io
            .get_input("A")
            .ok_or(NotGateError::MissingInput("A".to_string()))?
            .value;

        self.io
            .get_output_mut("B")
            .ok_or(NotGateError::MissingOutput("B".to_string()))?
            .set_value(!a);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tungsten_types::position::Position;

    use super::*;

    #[test]
    fn process() {
        let mut gate = NotGate::new(Position::ZERO);

        let truth_table: [(bool, bool); 2] = [(false, true), (true, false)];

        for (a, expected) in truth_table {
            gate.io.get_input_mut("A").unwrap().set_value(a);

            gate.process().expect("This should succeed");

            assert_eq!(gate.io.get_output("B").unwrap().value, expected);
        }
    }
}
