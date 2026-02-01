use crate::api::prelude::*;

mod operations;
pub use operations::*;

#[cfg(test)]
pub mod mock;

pub trait Bus<const W: usize>: Clone + Copy {
    fn eval(self) -> [WireState; W];
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}

#[derive(Clone, Copy)]
pub struct FanoutBus<const W: usize, WIRE: Wire> {
    wire: WIRE,
}

impl<const W: usize, WIRE: Wire> FanoutBus<W, WIRE> {
    pub fn new(wire: WIRE) -> Self {
        Self { wire }
    }
}

impl<const W: usize, WIRE: Wire> Bus<W> for FanoutBus<W, WIRE> {
    fn eval(self) -> [WireState; W] {
        [self.wire.eval(); W]
    }
}

#[derive(Clone, Copy)]
pub struct ConstBus<const W: usize> {
    values: [LogicalWireState; W],
}

impl<const W: usize> ConstBus<W> {
    pub fn new(values: [LogicalWireState; W]) -> Self {
        Self { values }
    }
}

impl<const W: usize> Bus<W> for ConstBus<W> {
    fn eval(self) -> [WireState; W] {
        self.values.map(From::from)
    }
}
