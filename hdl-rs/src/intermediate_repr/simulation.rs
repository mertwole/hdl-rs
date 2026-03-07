use std::collections::HashMap;

use crate::{api::prelude::*, intermediate_repr::{BusId, IntermediateRepr}};

pub enum SimulationEvent {
    Tick,
    CommitStateChanges,
}

pub struct Simulator {
    intermediate_repr: IntermediateRepr    
}

impl Simulator {
    pub fn new(intermediate_repr: IntermediateRepr) -> Self {
        Self {
            intermediate_repr
        }
    }

    pub fn simulate(&self, event: SimulationEvent) {
        //
    }
}

pub(super) struct SimulationContext {
    input_values: HashMap<BusId, Vec<WireState>>,
}

impl SimulationContext {
    pub(super) fn eval_bus(&self, bus_id: BusId) -> Vec<WireState> {
        if let Some(value) = self.input_values.get(&bus_id) {
            return value.clone();
        }

        todo!()
    }
}
