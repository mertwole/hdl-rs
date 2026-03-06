use crate::{
    api::prelude::*,
    intermediate_repr::{self, BusId, IntermediateReprBuilder},
};

use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

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

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus(B)]
pub struct InputBusWrapper<const W: usize, B: InputBus<W>> {
    bus: B,
    id: BusId,
}

impl<const W: usize, B: InputBus<W>> InputBusWrapper<W, B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus,
            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize, B: InputBus<W>> Bus<W> for InputBusWrapper<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.bus.eval()
    }

    fn get_id(self) -> BusId {
        self.bus.get_id()
    }

    fn build_intermediate_repr(self, builder: &mut intermediate_repr::IntermediateReprBuilder) {
        self.bus.build_intermediate_repr(builder);
    }
}

pub trait ClockBus {}

pub trait ResetBus {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus(B)]
pub struct FanoutBus<const W: usize, B: Bus<1>> {
    wire: B,
    id: BusId,
}

impl<const W: usize, B: Bus<1>> FanoutBus<W, B> {
    pub fn new(wire: B) -> Self {
        Self {
            wire,
            id: BusId::new_unique(W),
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
        builder.push_element(intermediate_repr::Gate::Unary(
            intermediate_repr::UnaryGate {
                input: self.wire.get_id(),
                output: self.id,
                operator: intermediate_repr::UnaryGateOperator::Fanout,
            },
        ));

        self.wire.build_intermediate_repr(builder);
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
pub struct ConstBus<const W: usize> {
    values: [LogicalWireState; W],
    id: BusId,
}

impl<const W: usize> ConstBus<W> {
    pub fn new(values: [LogicalWireState; W]) -> Self {
        Self {
            values,
            id: BusId::new_unique(W),
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
        builder.push_element(intermediate_repr::Gate::Const(
            intermediate_repr::ConstBus {
                id: self.id,
                value: self.values.to_vec(),
            },
        ));
    }
}
