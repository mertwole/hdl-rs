use autoimpl_operators::derive_bus_bitwise_ops;

use crate::api::prelude::*;

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FlipFlopBus<const W: usize, D: Bus<W>, C: Bus<1>, R: Bus<W>, S: Bus<W>> {
    current_state: [WireState; W],

    data: D,
    clock: C,
    reset: R,
    set: S,
}

impl<const W: usize, D: Bus<W>, C: Bus<1>, R: Bus<W>, S: Bus<W>> FlipFlopBus<W, D, C, R, S> {
    pub fn new(data: D, clock: C, reset: R, set: S) -> Self {
        Self {
            current_state: [WireState::X; W],
            data,
            clock,
            reset,
            set,
        }
    }

    pub fn resets_to_value<RST: Bus<1>>(
        data: D,
        clock: C,
        reset_wire: RST,
        reset_value: [LogicalWireState; W],
    ) -> FlipFlopBus<W, D, C, impl Bus<W>, impl Bus<W>> {
        let reset_fanout = FanoutBus::new(reset_wire);

        let set_mask = ConstBus::new(reset_value);
        let set_bus = reset_fanout.and(set_mask);

        let reset_mask = ConstBus::new(reset_value.map(|value| !value));
        let reset_bus = reset_fanout.and(reset_mask);

        FlipFlopBus::new(data, clock, reset_bus, set_bus)
    }
}

impl<const W: usize, D: Bus<W>, C: Bus<1>, R: Bus<W>, S: Bus<W>> Bus<W>
    for FlipFlopBus<W, D, C, R, S>
{
    const COMBINATIONAL_NETWORK_ID: usize = 1 + usize_min_4(
        D::COMBINATIONAL_NETWORK_ID,
        C::COMBINATIONAL_NETWORK_ID,
        R::COMBINATIONAL_NETWORK_ID,
        S::COMBINATIONAL_NETWORK_ID,
    );

    fn eval(self) -> [WireState; W] {
        self.current_state
    }
}

const fn usize_min_4(a: usize, b: usize, c: usize, d: usize) -> usize {
    usize_min(usize_min(a, b), usize_min(c, d))
}

const fn usize_min(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs { lhs } else { rhs }
}
