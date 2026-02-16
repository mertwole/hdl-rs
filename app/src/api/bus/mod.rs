use crate::{
    api::prelude::*,
    intermediate_repr::{self, BusId, IntermediateReprBuilder},
};

use autoimpl_operators::{derive_bus_bitwise_ops, derive_clock_bus, derive_reset_bus};

mod operations;
pub use operations::*;

mod feedback;
#[allow(unused_imports)]
pub use feedback::*;

#[cfg(test)]
pub mod mock;

pub trait Bus<const W: usize>: BusOpsMarker + Clone + Copy {
    const COMBINATIONAL_NETWORK_ID: usize;

    fn eval(self) -> [WireState; W];

    fn get_id(self) -> BusId;

    fn build_intermediate_repr(self, builder: &mut IntermediateReprBuilder);
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}

pub trait ClockBus {}

pub trait ResetBus {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus(B)]
#[derive_reset_bus(B)]
pub struct FanoutBus<const W: usize, B: Bus<1>> {
    wire: B,
    id: BusId,
}

impl<const W: usize, B: Bus<1>> FanoutBus<W, B> {
    pub fn new(wire: B) -> Self {
        Self {
            wire,
            id: BusId::new(W),
        }
    }
}

impl<const W: usize, B: Bus<1>> Bus<W> for FanoutBus<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; W] {
        [self.wire.eval()[0]; W]
    }

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(self, builder: &mut IntermediateReprBuilder) {
        builder.push_element(
            intermediate_repr::connections::FanoutBus {
                output_width: W,
                input: self.wire.get_id(),
                output: self.id,
            },
            self.id,
        );

        self.wire.build_intermediate_repr(builder);
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
#[derive_reset_bus]
pub struct ConstBus<const W: usize> {
    values: [LogicalWireState; W],
    id: BusId,
}

impl<const W: usize> ConstBus<W> {
    pub fn new(values: [LogicalWireState; W]) -> Self {
        Self {
            values,
            id: BusId::new(W),
        }
    }
}

impl<const W: usize> Bus<W> for ConstBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.values.map(From::from)
    }

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(self, builder: &mut IntermediateReprBuilder) {
        builder.push_element(
            intermediate_repr::ConstBus {
                id: self.id,
                width: W,
                value: self.values.to_vec(),
            },
            self.id,
        );
    }
}
