use std::collections::HashMap;

use tungsten_types::{net::NetId, nodes::NodeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateConnection {
    None,
    Net(NetId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateValue {
    pub name: String,
    pub value: NodeValue,
    pub net: Option<GateConnection>,
}
impl GateValue {
    pub fn new(name: &str, value: NodeValue) -> Self {
        Self {
            name: name.to_string(),
            value,
            net: None,
        }
    }
}

impl GateValue {
    pub fn set_value(&mut self, value: NodeValue) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateIO {
    inputs: Vec<GateValue>,
    outputs: Vec<GateValue>,
    input_map: HashMap<String, usize>,
    output_map: HashMap<String, usize>,
}

impl GateIO {
    pub fn new(inputs: Vec<GateValue>, outputs: Vec<GateValue>) -> Self {
        let input_map = inputs
            .iter()
            .enumerate()
            .map(|(i, input)| (input.name.clone(), i))
            .collect();
        let output_map = outputs
            .iter()
            .enumerate()
            .map(|(i, output)| (output.name.clone(), i))
            .collect();
        Self {
            inputs,
            outputs,
            input_map,
            output_map,
        }
    }
}

impl GateIO {
    pub fn get_input(&self, name: &str) -> Option<&GateValue> {
        self.input_map.get(name).map(|&index| &self.inputs[index])
    }

    pub fn get_input_mut(&mut self, name: &str) -> Option<&mut GateValue> {
        self.input_map
            .get_mut(name)
            .map(|&mut index| &mut self.inputs[index])
    }

    pub fn get_output(&self, name: &str) -> Option<&GateValue> {
        self.output_map.get(name).map(|&index| &self.outputs[index])
    }

    pub fn get_output_mut(&mut self, name: &str) -> Option<&mut GateValue> {
        self.output_map
            .get_mut(name)
            .map(|&mut index| &mut self.outputs[index])
    }

    pub fn inputs_mut(&mut self) -> &mut [GateValue] {
        &mut self.inputs
    }

    pub fn outputs(&self) -> &[GateValue] {
        &self.outputs
    }
}

pub trait GateNode<E: std::error::Error> {
    fn process(&mut self) -> Result<(), E>;
}
