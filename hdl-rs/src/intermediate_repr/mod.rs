use std::collections::{HashMap, HashSet, hash_map::Entry};

use crate::{
    api::prelude::{LogicalWireState, WireState},
    intermediate_repr::simulation::SimulationContext,
    verilog::{self, VerilogModule},
};

mod bus_id;
pub use bus_id::*;

mod simulation;

pub struct IntermediateReprBuilder {
    repr: IntermediateRepr,
}

#[derive(Default, Clone)]
pub struct IntermediateRepr {
    nodes: HashMap<BusId, Gate>,
    outputs: HashSet<OutputBus>,
}

impl IntermediateReprBuilder {
    pub fn new() -> Self {
        Self {
            repr: Default::default(),
        }
    }

    pub fn push_element(&mut self, element: Gate) {
        if let Entry::Vacant(entry) = self.repr.nodes.entry(element.get_id()) {
            entry.insert(element);
        }
    }

    pub fn push_output(&mut self, output: BusId) {
        self.repr.outputs.insert(OutputBus { id: output });
    }

    pub fn build(self) -> IntermediateRepr {
        self.repr
    }
}

impl IntermediateRepr {
    // TODO: Move this fn to `VerilogModule::from_intermediate_repr`.
    pub fn to_verilog(&self) -> VerilogModule {
        let mut module = VerilogModule::new();

        for node in self.nodes.values() {
            node.to_verilog(&mut module);
        }

        for output in &self.outputs {
            output.to_verilog(&mut module);
        }

        module
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OutputBus {
    pub id: BusId,
}

impl OutputBus {
    fn to_verilog(self, module: &mut VerilogModule) {
        module.add_output(verilog::OutputWire {
            name: self.id.to_string(),
            output_name: format!("{}_output", self.id),
            width: self.id.width(),
        });
    }
}

#[derive(Clone, Debug)]
pub enum Gate {
    Input(InputBus),
    Const(ConstBus),
    Unary(UnaryGate),
    Binary(BinaryGate),
    FlipFlop(FlipFlop),
}

impl Gate {
    fn get_id(&self) -> BusId {
        match self {
            Self::Input(input) => input.id,
            Self::Const(const_bus) => const_bus.id,
            Self::Unary(unary) => unary.output,
            Self::Binary(binary) => binary.output,
            Self::FlipFlop(flip_flop) => flip_flop.output,
        }
    }

    fn to_verilog(&self, module: &mut VerilogModule) {
        match self {
            Self::Input(input) => input.to_verilog(module),
            Self::Const(const_bus) => const_bus.to_verilog(module),
            Self::Unary(unary) => unary.to_verilog(module),
            Self::Binary(binary) => binary.to_verilog(module),
            Self::FlipFlop(flip_flop) => flip_flop.to_verilog(module),
        }
    }

    fn eval(&self, ctx: &SimulationContext) -> Vec<WireState> {
        match self {
            Self::Input(input) => input.eval(ctx),
            Self::Const(const_bus) => const_bus.eval(ctx),
            Self::Unary(unary) => unary.eval(ctx),
            Self::Binary(binary) => binary.eval(ctx),
            Self::FlipFlop(flip_flop) => flip_flop.eval(ctx),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputBus {
    pub id: BusId,
}

impl InputBus {
    fn to_verilog(&self, module: &mut VerilogModule) {
        module.add_input(verilog::InputWire {
            name: self.id.to_string(),
            width: self.id.width(),
        });
    }

    fn eval(&self, ctx: &SimulationContext) -> Vec<WireState> {
        ctx.eval_bus(self.id)
    }
}

#[derive(Clone, Debug)]
pub struct ConstBus {
    pub id: BusId,
    pub value: Vec<LogicalWireState>,
}

impl ConstBus {
    fn to_verilog(&self, module: &mut VerilogModule) {
        let value: Vec<_> = self.value.iter().copied().map(From::from).collect();

        let wire = verilog::WireDefinition {
            name: self.id.to_string(),
            width: self.id.width(),
            assignment: Some(verilog::Expression::Const { value }),
        };

        module.add_wire(wire);
    }

    fn eval(&self, _ctx: &SimulationContext) -> Vec<WireState> {
        self.value.iter().copied().map(Into::into).collect()
    }
}

#[derive(Clone, Debug)]
pub struct BinaryGate {
    pub lhs: BusId,
    pub rhs: BusId,
    pub output: BusId,
    pub operator: BinaryGateOperator,
}

#[derive(Clone, Debug)]
pub enum BinaryGateOperator {
    And,
    Or,
    Xor,
    Concat,
}

impl BinaryGate {
    fn to_verilog(&self, module: &mut VerilogModule) {
        match self.operator {
            BinaryGateOperator::And => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::And {
                        lhs: self.lhs.to_string(),
                        rhs: self.rhs.to_string(),
                    }),
                });
            }
            BinaryGateOperator::Or => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::Or {
                        lhs: self.lhs.to_string(),
                        rhs: self.rhs.to_string(),
                    }),
                });
            }
            BinaryGateOperator::Xor => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::Xor {
                        lhs: self.lhs.to_string(),
                        rhs: self.rhs.to_string(),
                    }),
                });
            }
            BinaryGateOperator::Concat => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::Concat {
                        lhs: self.lhs.to_string(),
                        rhs: self.rhs.to_string(),
                    }),
                });
            }
        }
    }

    fn eval(&self, ctx: &SimulationContext) -> Vec<WireState> {
        let mut lhs = ctx.eval_bus(self.lhs);
        let mut rhs = ctx.eval_bus(self.rhs);

        let zipped = lhs.iter().zip(rhs.iter());

        let result: Vec<_> = match self.operator {
            BinaryGateOperator::And => {
                zipped.map(|(lhs, rhs)| lhs.and(*rhs)).collect()
            }
            BinaryGateOperator::Or => {
                zipped.map(|(lhs, rhs)| lhs.or(*rhs)).collect()
            }
            BinaryGateOperator::Xor => {
                zipped.map(|(lhs, rhs)| lhs.xor(*rhs)).collect()
            }
            BinaryGateOperator::Concat => {
                lhs.append(&mut rhs);
                lhs
            }
        };

        assert_eq!(result.len(), self.output.width());
        result
    }
}

#[derive(Clone, Debug)]
pub struct UnaryGate {
    pub input: BusId,
    pub output: BusId,
    pub operator: UnaryGateOperator,
}

#[derive(Clone, Debug)]
pub enum UnaryGateOperator {
    Not,
    ShiftLeft { shift: usize },
    ShiftRight { shift: usize },
    SubBus { from: usize, to: usize },
    Fanout,
}

impl UnaryGate {
    fn to_verilog(&self, module: &mut VerilogModule) {
        match self.operator {
            UnaryGateOperator::Not => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::Not {
                        wire: self.input.to_string(),
                    }),
                });
            }
            UnaryGateOperator::ShiftLeft { shift } => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::LeftShift {
                        wire: self.input.to_string(),
                        amount: shift,
                    }),
                });
            }
            UnaryGateOperator::ShiftRight { shift } => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::RightShift {
                        wire: self.input.to_string(),
                        amount: shift,
                    }),
                });
            }
            UnaryGateOperator::SubBus { from, to } => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    // TODO: Properly convert range to the verilog indexes.
                    assignment: Some(verilog::Expression::Range {
                        wire: self.input.to_string(),
                        from,
                        to,
                    }),
                });
            }
            UnaryGateOperator::Fanout => {
                module.add_wire(verilog::WireDefinition {
                    name: self.output.to_string(),
                    width: self.output.width(),
                    assignment: Some(verilog::Expression::Fanout {
                        wire: self.input.to_string(),
                        output_width: self.output.width(),
                    }),
                });
            }
        }
    }

    fn eval(&self, ctx: &SimulationContext) -> Vec<WireState> {
        let input = ctx.eval_bus(self.input);

        let result = match self.operator {
            UnaryGateOperator::Not => input.into_iter().map(|value| value.not()).collect(),
            UnaryGateOperator::ShiftLeft { shift } => (0..self.output.width())
                .map(|i| {
                    if i + shift >= self.output.width() {
                        WireState::Zero
                    } else {
                        input[i + shift]
                    }
                })
                .collect(),
            UnaryGateOperator::ShiftRight { shift } => (0..self.output.width())
                .map(|i| {
                    if shift > i {
                        WireState::Zero
                    } else {
                        input[i - shift]
                    }
                })
                .collect(),
            UnaryGateOperator::SubBus { from, to } => {
                input[self.input.width() - 1 - from..self.input.width() - to].to_vec()
            }
            UnaryGateOperator::Fanout => {
                vec![input[0]; self.output.width()]
            }
        };

        assert_eq!(result.len(), self.output.width());

        result
    }
}

#[derive(Clone, Debug)]
pub struct FlipFlop {
    pub data: BusId,
    pub clock: BusId,
    pub output: BusId,
}

impl FlipFlop {
    fn to_verilog(&self, module: &mut VerilogModule) {
        module.add_register(verilog::RegisterDefinition {
            name: self.output.to_string(),
            width: self.output.width(),
            clock: self.clock.to_string(),
            data_bus: self.data.to_string(),
        });
    }

    fn eval(&self, _ctx: &SimulationContext) -> Vec<WireState> {
        todo!()
    }
}
