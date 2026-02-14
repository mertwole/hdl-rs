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
        self.wires
            .iter()
            .map(|wire| match &wire.assignment {
                Some(assignment) => {
                    format!(
                        "wire [{} - 1:0] {} = {};",
                        wire.width,
                        wire.name,
                        assignment.to_verilog()
                    )
                }
                None => {
                    format!("wire [{} - 1:0] {};", wire.width, wire.name)
                }
            })
            .intersperse(String::from("\n"))
            .collect()
    }

    fn generate_reg_definitions(&self) -> String {
        self.registers
            .iter()
            .map(|reg| format!("reg [{} - 1:0] {};", reg.width, reg.name))
            .intersperse(String::from("\n"))
            .collect()
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

impl Expression {
    fn to_verilog(&self) -> String {
        match self {
            Self::Const { value } => String::from("TODO"),
            Self::And { lhs, rhs } => format!("{lhs} & {rhs}"),
            Self::Or { lhs, rhs } => format!("{lhs} | {rhs}"),
            Self::Xor { lhs, rhs } => format!("{lhs} ^ {rhs}"),
            Self::Not { wire } => format!("~{wire}"),
            Self::LeftShift { wire, amount } => String::from("TODO"),
            Self::RightShift { wire, amount } => String::from("TODO"),
            Self::Range { wire, from, to } => String::from("TODO"),
            Self::Concat { lhs, rhs } => format!("{{{lhs}, {rhs}}}"),
        }
    }
}
