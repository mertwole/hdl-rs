use derive_macros::derive_bus_bitwise_ops;

use crate::{
    api::prelude::*,
    intermediate_repr::{self, BusId},
};

// TODO: Decide what bounds should be applied when implementing ClockBus and ResetBus for FlipFlopBus.
// TODO: Implement `#[diagnostic::on_unimplemented]` to clarify the `ClockBus` and `ResetBus` bounds behaviour.
#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FlipFlopBus<const W: usize, D: Bus<W>, C: Bus<1> + ClockBus> {
    current_state: [WireState; W],

    data: D,
    clock: C,

    id: BusId,
}

impl<const W: usize, D: Bus<W>, C: Bus<1> + ClockBus> FlipFlopBus<W, D, C> {
    pub fn new(data: D, clock: C) -> Self {
        Self {
            current_state: [WireState::X; W],
            data,
            clock,

            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize, D: Bus<W>, C: Bus<1> + ClockBus> Bus<W> for FlipFlopBus<W, D, C> {
    const COMBINATIONAL_NETWORK_ID: usize =
        1 + usize_min(D::COMBINATIONAL_NETWORK_ID, C::COMBINATIONAL_NETWORK_ID);

    fn eval(self) -> [WireState; W] {
        self.current_state
    }

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(
        self,
        builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
        builder.push_element(intermediate_repr::Gate::FlipFlop(
            intermediate_repr::FlipFlop {
                data: self.data.get_id(),
                clock: self.clock.get_id(),
                output: self.id,
            },
        ));

        self.data.build_intermediate_repr(builder);
        self.clock.build_intermediate_repr(builder);
    }
}

const fn usize_min(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs { lhs } else { rhs }
}
