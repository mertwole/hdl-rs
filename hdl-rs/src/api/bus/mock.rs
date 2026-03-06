use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

use crate::{
    api::{
        bus::InputBus,
        prelude::{Bus, WireState},
    },
    intermediate_repr::{BusId, IntermediateReprBuilder},
};

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
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

    fn get_id(self) -> BusId {
        BusId::mock()
    }

    fn build_intermediate_repr(self, _builder: &mut IntermediateReprBuilder) {
        unimplemented!()
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
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

    fn get_id(self) -> BusId {
        BusId::mock()
    }

    fn build_intermediate_repr(self, _builder: &mut IntermediateReprBuilder) {
        unimplemented!()
    }
}

impl<const W: usize> InputBus<W> for MockInputBus<W> {}
