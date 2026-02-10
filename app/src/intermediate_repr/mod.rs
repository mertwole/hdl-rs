use std::sync::{Arc, Mutex, OnceLock};

pub mod connections;
pub mod flip_flop;
pub mod gates;

pub trait IntermediateRepr {}

pub struct InputBus {
    pub width: usize,
    pub id: BusId,
}

impl IntermediateRepr for InputBus {}

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

#[derive(Clone, Copy)]
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
}

pub struct Module {
    inputs: Vec<BusId>,
    outputs: Vec<BusId>,
}

pub struct IntermediateReprBuilder {}

impl IntermediateReprBuilder {
    pub fn push_element(&mut self, element: impl IntermediateRepr, id: BusId) {
        //
    }
}
