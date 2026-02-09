use crate::intermediate_repr::IntermediateRepr;

use super::BusId;

pub struct FlipFlop {
    pub data: BusId,
    pub reset: BusId,
    pub set: BusId,
    pub clock: BusId,

    pub output: BusId,
}

impl IntermediateRepr for FlipFlop {}
