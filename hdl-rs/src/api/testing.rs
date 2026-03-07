use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

use crate::{
    api::prelude::*,
    intermediate_repr::{self, BusId},
};

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
pub struct TestInputBus<const W: usize> {
    value: [WireState; W],
    id: BusId,
}

impl<const W: usize> TestInputBus<W> {
    pub fn new(value: [WireState; W]) -> Self {
        Self {
            value,
            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize> InputBus<W> for TestInputBus<W> {}

impl<const W: usize> Bus<W> for TestInputBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(self, builder: &mut intermediate_repr::IntermediateReprBuilder) {
        builder.push_element(intermediate_repr::Gate::Input(
            intermediate_repr::InputBus { id: self.id },
        ));
    }
}
