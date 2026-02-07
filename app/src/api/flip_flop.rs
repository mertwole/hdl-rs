use autoimpl_operators::derive_bus_bitwise_ops;

use crate::api::prelude::*;

#[derive(Clone, Copy)]
pub struct FlipFlop<D: Wire, C: Wire, R: Wire, S: Wire> {
    current_state: WireState,

    data: D,
    clock: C,
    reset: R,
    set: S,
}

impl<D: Wire, C: Wire, R: Wire, S: Wire> FlipFlop<D, C, R, S> {
    pub fn new(data: D, clock: C, reset: R, set: S) -> Self {
        Self {
            current_state: WireState::X,
            data,
            clock,
            reset,
            set,
        }
    }

    pub fn resets_to_zero(data: D, clock: C, reset_wire: R) -> FlipFlop<D, C, R, ConstZeroWire> {
        FlipFlop::new(data, clock, reset_wire, ConstZeroWire::new())
    }

    pub fn resets_to_one(data: D, clock: C, reset_wire: S) -> FlipFlop<D, C, ConstZeroWire, S> {
        FlipFlop::new(data, clock, ConstZeroWire::new(), reset_wire)
    }
}

impl<D: Wire, C: Wire, R: Wire, S: Wire> Wire for FlipFlop<D, C, R, S> {
    fn eval(&self) -> WireState {
        self.current_state
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FlipFlopBus<const W: usize, D: Bus<W>, C: Wire, R: Bus<W>, S: Bus<W>> {
    current_state: [WireState; W],

    data: D,
    clock: C,
    reset: R,
    set: S,
}

impl<const W: usize, D: Bus<W>, C: Wire, R: Bus<W>, S: Bus<W>> FlipFlopBus<W, D, C, R, S> {
    pub fn new(data: D, clock: C, reset: R, set: S) -> Self {
        Self {
            current_state: [WireState::X; W],
            data,
            clock,
            reset,
            set,
        }
    }

    pub fn resets_to_value<WIRE: Wire>(
        data: D,
        clock: C,
        reset_wire: WIRE,
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

impl<const W: usize, D: Bus<W>, C: Wire, R: Bus<W>, S: Bus<W>> Bus<W>
    for FlipFlopBus<W, D, C, R, S>
{
    fn eval(self) -> [WireState; W] {
        self.current_state
    }
}
