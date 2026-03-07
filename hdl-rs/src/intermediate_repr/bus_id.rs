use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex, OnceLock},
};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
