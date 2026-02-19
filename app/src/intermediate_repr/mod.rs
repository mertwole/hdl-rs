use std::collections::{HashMap, hash_map::Entry};

use crate::{api::prelude::LogicalWireState, verilog::VerilogModule};

mod bus_id;
pub use bus_id::*;

pub struct IntermediateReprBuilder {
    nodes: HashMap<BusId, Gate>,
}

// TODO: Add method `finalize` which will return `IntermediateRepr`.
impl IntermediateReprBuilder {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn push_element(&mut self, element: Gate) {
        if let Entry::Vacant(entry) = self.nodes.entry(element.get_id()) {
            entry.insert(element);
        }
    }

    // TODO: Move this fn to `VerilogModule::from_intermediate_repr`.
    pub fn to_verilog(&self) -> VerilogModule {
        let mut module = VerilogModule::new();

        for node in self.nodes.values() {
            node.to_verilog(&mut module);
        }

        module
    }
}

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
        todo!()
    }
}

pub struct InputBus {
    pub id: BusId,
}

pub struct ConstBus {
    pub id: BusId,
    pub value: Vec<LogicalWireState>,
}

pub struct BinaryGate {
    pub lhs: BusId,
    pub rhs: BusId,
    pub output: BusId,
    pub operator: BinaryGateOperator,
}

pub enum BinaryGateOperator {
    And,
    Or,
    Xor,
    Concat,
}

pub struct UnaryGate {
    pub input: BusId,
    pub output: BusId,
    pub operator: UnaryGateOperator,
}

pub enum UnaryGateOperator {
    Not,
    ShiftLeft { shift: usize },
    ShiftRight { shift: usize },
    SubBus { from: usize, to: usize },
    Fanout,
}

pub struct FlipFlop {
    pub data: BusId,
    // TODO: Process `reset` and `set`.
    pub reset: BusId,
    pub set: BusId,

    pub clock: BusId,

    pub output: BusId,
}
