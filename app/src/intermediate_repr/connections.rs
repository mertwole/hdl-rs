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

impl IntermediateRepr for BusConcat {}

pub struct FanoutBus {
    pub input: BusId,
    pub output: BusId,
}

impl IntermediateRepr for FanoutBus {}

pub struct BusShiftLeft {
    pub input: BusId,
    pub output: BusId,
    pub shift: usize,
}

impl IntermediateRepr for BusShiftLeft {}

pub struct BusShiftRight {
    pub input: BusId,
    pub output: BusId,
    pub shift: usize,
}

impl IntermediateRepr for BusShiftRight {}
