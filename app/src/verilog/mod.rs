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

    pub fn generate_verilog(&self) -> String {
        let module_io = self.generate_module_io();
        let wire_defs = self.generate_wire_definitions();
        let reg_defs = self.generate_reg_definitions();

        format!(
            "
        modulue test(\n\
        {module_io}\n\
        );
        \n\
        {wire_defs}\n\
        \n\
        {reg_defs}\n\
        \n\
        endmodule
        "
        )
    }

    fn generate_module_io(&self) -> String {
        let inputs = self
            .inputs
            .iter()
            .map(|input| format!("input [{} - 1:0] {}", input.width, input.name));

        let outputs = self
            .outputs
            .iter()
            .map(|output| format!("output wire [{} - 1:0] {}", output.width, output.name));

        inputs
            .chain(outputs)
            .intersperse(String::from(",\n"))
            .collect()
    }

    fn generate_wire_definitions(&self) -> String {
        format!("")
    }

    fn generate_reg_definitions(&self) -> String {
        format!("")
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
    Const {
        value: Vec<bool>,
    },
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
