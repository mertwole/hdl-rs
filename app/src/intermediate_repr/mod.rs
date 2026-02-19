use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::{Binary, Display, Formatter},
    hash::Hash,
    process::Output,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{
    api::prelude::LogicalWireState,
    verilog::{self, VerilogModule},
};

pub enum Gate {
    Input(InputBus),
    Const(ConstBus),
    Unary(UnaryGate),
    Binary(BinaryGate),
    FlipFlop(FlipFlop),
}

impl Gate {
    fn get_id(&self) -> BusId {
        todo!()
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

static ID_REGISTRY: OnceLock<IdRegistry> = OnceLock::new();

pub struct IdRegistry {
    last_id: Arc<Mutex<usize>>,
}

impl IdRegistry {
    fn new() -> Self {
        Self {
            last_id: Arc::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BusId {
    id: usize,
    width: usize,
}

impl BusId {
    pub fn new_unique(width: usize) -> Self {
        let mut id = ID_REGISTRY
            .get_or_init(IdRegistry::new)
            .last_id
            .lock()
            .expect("Concurrency is not expected");
        let new_id = *id;
        *id += 1;

        Self { id: new_id, width }
    }

    pub fn width(self) -> usize {
        self.width
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self { id: 0, width: 0 }
    }
}

// TODO: Remove it. These names shouldn't appear on schematic and in verilog code.
impl Display for BusId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bus_{}", self.id)
    }
}

pub struct Module {
    inputs: Vec<BusId>,
    outputs: Vec<BusId>,
}

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
