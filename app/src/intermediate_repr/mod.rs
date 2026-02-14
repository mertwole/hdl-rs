use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::{Display, Formatter},
    hash::Hash,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{
    api::prelude::LogicalWireState,
    verilog::{self, VerilogModule},
};

pub mod connections;
pub mod flip_flop;
pub mod gates;

pub trait IntermediateRepr {
    fn to_verilog(&self, module: &mut VerilogModule);
}

pub struct InputBus {
    pub width: usize,
    pub id: BusId,
}

impl IntermediateRepr for InputBus {
    fn to_verilog(&self, module: &mut VerilogModule) {
        module.add_input(verilog::InputWire {
            name: self.id.to_string(),
            width: self.width,
        });
    }
}

pub struct ConstBus {
    pub width: usize,
    pub id: BusId,
    pub value: Vec<LogicalWireState>,
}

impl IntermediateRepr for ConstBus {
    fn to_verilog(&self, module: &mut VerilogModule) {
        let value: Vec<_> = self.value.iter().copied().map(From::from).collect();

        let wire = verilog::WireDefinition {
            name: self.id.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::Const { value }),
        };

        module.add_wire(wire);
    }
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
}

impl BusId {
    // TODO: Rename to `new_unique`;
    pub fn new() -> Self {
        let mut id = ID_REGISTRY
            .get_or_init(IdRegistry::new)
            .last_id
            .lock()
            .expect("Concurrency is not expected");
        let new_id = *id;
        *id += 1;

        Self { id: new_id }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self { id: 0 }
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
    nodes: HashMap<BusId, Box<dyn IntermediateRepr>>,
}

// TODO: Add method `finalize` which will return `IntermediateRepr`.
impl IntermediateReprBuilder {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn push_element(&mut self, element: impl IntermediateRepr + 'static, id: BusId) {
        if let Entry::Vacant(entry) = self.nodes.entry(id) {
            entry.insert(Box::from(element));
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
