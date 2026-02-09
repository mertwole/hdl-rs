use crate::{
    api::prelude::*,
    intermediate_repr::{BusId, IntermediateRepr},
};

use autoimpl_operators::derive_bus_bitwise_ops;

mod operations;
pub use operations::*;

mod feedback;
pub use feedback::*;

#[cfg(test)]
pub mod mock;

pub trait Bus<const W: usize>: Clone + Copy {
    const COMBINATIONAL_NETWORK_ID: usize;

    fn eval(self) -> [WireState; W];

    fn get_id(self) -> BusId;

    fn construct_intermediate_repr(self) -> impl IntermediateRepr;
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FanoutBus<const W: usize, B: Bus<1>> {
    wire: B,
}

impl<const W: usize, B: Bus<1>> FanoutBus<W, B> {
    pub fn new(wire: B) -> Self {
        Self { wire }
    }
}

impl<const W: usize, B: Bus<1>> Bus<W> for FanoutBus<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; W] {
        [self.wire.eval()[0]; W]
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct ConstBus<const W: usize> {
    values: [LogicalWireState; W],
}

impl<const W: usize> ConstBus<W> {
    pub fn new(values: [LogicalWireState; W]) -> Self {
        Self { values }
    }
}

impl<const W: usize> Bus<W> for ConstBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.values.map(From::from)
    }
}
