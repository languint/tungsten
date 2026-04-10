use tungsten_types::nodes::NodeValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateValue {
    pub name: String,
    pub value: NodeValue,
}
impl GateValue {
    pub fn new(name: &str, value: NodeValue) -> Self {
        Self {
            name: name.to_string(),
            value,
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
    pub inputs: Vec<GateValue>,
    pub outputs: Vec<GateValue>,
}

impl GateIO {
    pub fn new(inputs: Vec<GateValue>, outputs: Vec<GateValue>) -> Self {
        Self { inputs, outputs }
    }
}

impl GateIO {
    pub fn get_input(&self, name: &str) -> Option<&GateValue> {
        self.inputs.iter().find(|input| input.name == name)
    }

    pub fn get_input_mut(&mut self, name: &str) -> Option<&mut GateValue> {
        self.inputs.iter_mut().find(|input| input.name == name)
    }

    pub fn get_output(&self, name: &str) -> Option<&GateValue> {
        self.outputs.iter().find(|output| output.name == name)
    }

    pub fn get_output_mut(&mut self, name: &str) -> Option<&mut GateValue> {
        self.outputs.iter_mut().find(|output| output.name == name)
    }
}

pub trait GateNode<E: std::error::Error> {
    fn process(&mut self) -> Result<(), E>;
}
