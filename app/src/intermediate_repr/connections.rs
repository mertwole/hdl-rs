use crate::intermediate_repr::{BusId, IntermediateRepr};

pub struct SubBus {
    pub input: BusId,
    pub from: usize,
    pub to: usize,
    pub output: BusId,
}

impl IntermediateRepr for SubBus {}

pub struct BusConcat {
    pub lhs: BusId,
    pub rhs: BusId,
    pub output: BusId,
}

impl IntermediateRepr for SubBus {}
