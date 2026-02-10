use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, Mutex, OnceLock},
};

use crate::api::prelude::LogicalWireState;

pub mod connections;
pub mod flip_flop;
pub mod gates;

pub trait IntermediateRepr {}

pub struct InputBus {
    pub width: usize,
    pub id: BusId,
}

impl IntermediateRepr for InputBus {}

pub struct ConstBus {
    pub width: usize,
    pub id: BusId,
    pub value: Vec<LogicalWireState>,
}

impl IntermediateRepr for ConstBus {}

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

pub struct Module {
    inputs: Vec<BusId>,
    outputs: Vec<BusId>,
}

pub struct IntermediateReprBuilder {
    nodes: HashMap<BusId, Box<dyn IntermediateRepr>>,
}

impl IntermediateReprBuilder {
    pub fn push_element(&mut self, element: impl IntermediateRepr + 'static, id: BusId) {
        if let Entry::Vacant(entry) = self.nodes.entry(id) {
            entry.insert(Box::from(element));
        }
    }
}
