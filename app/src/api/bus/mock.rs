use autoimpl_operators::derive_bus_bitwise_ops;

use crate::{
    api::{
        bus::{HasBusId, InputBus},
        prelude::{Bus, WireState},
    },
    intermediate_repr::{BusId, IntermediateReprBuilder},
};

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct MockBus<const W: usize>([WireState; W]);

impl<const W: usize> MockBus<W> {
    pub fn new(values: [WireState; W]) -> Self {
        Self(values)
    }
}

impl<const W: usize> Bus<W> for MockBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.0
    }

    fn build_intermediate_repr(self, _builder: &mut IntermediateReprBuilder) {
        unimplemented!()
    }
}

impl<const W: usize> HasBusId for MockBus<W> {
    fn get_id(self) -> BusId {
        BusId::mock()
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct MockInputBus<const W: usize> {
    values: [WireState; W],
}

impl<const W: usize> MockInputBus<W> {
    pub fn new(values: [WireState; W]) -> Self {
        Self { values }
    }
}

impl<const W: usize> Bus<W> for MockInputBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.values
    }

    fn build_intermediate_repr(self, _builder: &mut IntermediateReprBuilder) {
        unimplemented!()
    }
}

impl<const W: usize> HasBusId for MockInputBus<W> {
    fn get_id(self) -> BusId {
        BusId::mock()
    }
}

impl<const W: usize> InputBus<W> for MockInputBus<W> {}
