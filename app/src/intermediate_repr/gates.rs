use crate::intermediate_repr::IntermediateRepr;

use super::BusId;

pub struct And {
    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for And {}

pub struct Or {
    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Or {}

pub struct Xor {
    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Xor {}

pub struct Not {
    pub bus: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Not {}
