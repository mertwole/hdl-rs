pub struct VerilogModule {
    inputs: Vec<InputWire>,
    outputs: Vec<OutputWire>,

    registers: Vec<RegisterDefinition>,
    wires: Vec<WireDefinition>,
}

impl VerilogModule {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            outputs: vec![],
            registers: vec![],
            wires: vec![],
        }
    }

    pub fn add_input(&mut self, input: InputWire) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: OutputWire) {
        self.outputs.push(output);
    }

    pub fn add_wire(&mut self, wire: WireDefinition) {
        self.wires.push(wire);
    }

    pub fn add_register(&mut self, register: RegisterDefinition) {
        self.registers.push(register);
    }
}

pub struct InputWire {
    pub name: String,
    pub width: usize,
}

pub struct OutputWire {
    pub name: String,
    pub width: usize,
}

pub struct RegisterDefinition {
    pub name: String,
    pub width: usize,
}

pub struct WireDefinition {
    pub name: String,
    pub width: usize,
    pub assignment: Option<Expression>,
}

pub enum Expression {
    And {
        lhs: String,
        rhs: String,
    },
    Or {
        lhs: String,
        rhs: String,
    },
    Xor {
        lhs: String,
        rhs: String,
    },
    Not {
        wire: String,
    },
    LeftShift {
        wire: String,
        amount: usize,
    },
    RightShift {
        wire: String,
        amount: usize,
    },
    Range {
        wire: String,
        from: usize,
        to: usize,
    },
    Concat {
        lhs: String,
        rhs: String,
    },
}
