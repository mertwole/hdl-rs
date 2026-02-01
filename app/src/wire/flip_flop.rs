use crate::wire::{
    ConstZeroWire, LogicalWireState, Wire, WireState,
    bus::{Bus, ConstBus, FanoutBus},
    bus_operators::BusOps,
};

#[derive(Clone, Copy)]
pub struct FlipFlop<D: Wire, R: Wire, S: Wire> {
    current_state: WireState,

    data: D,
    reset: R,
    set: S,
}

impl<D: Wire, R: Wire, S: Wire> FlipFlop<D, R, S> {
    pub fn new(data: D, reset: R, set: S) -> Self {
        Self {
            current_state: WireState::X,
            data,
            reset,
            set,
        }
    }

    pub fn resets_to_zero(data: D, reset_wire: R) -> FlipFlop<D, R, ConstZeroWire> {
        FlipFlop::new(data, reset_wire, ConstZeroWire::new())
    }

    pub fn resets_to_one(data: D, reset_wire: S) -> FlipFlop<D, ConstZeroWire, S> {
        FlipFlop::new(data, ConstZeroWire::new(), reset_wire)
    }
}

impl<D: Wire, R: Wire, S: Wire> Wire for FlipFlop<D, R, S> {
    fn eval(&self) -> WireState {
        self.current_state
    }
}

#[derive(Clone, Copy)]
struct FlipFlopBus<const W: usize, D: Bus<W>, R: Bus<W>, S: Bus<W>> {
    current_state: [WireState; W],

    data: D,
    reset: R,
    set: S,
}

impl<const W: usize, D: Bus<W>, R: Bus<W>, S: Bus<W>> FlipFlopBus<W, D, R, S> {
    pub fn new(data: D, reset: R, set: S) -> Self {
        Self {
            current_state: [WireState::X; W],
            data,
            reset,
            set,
        }
    }

    pub fn resets_to_value<WIRE: Wire>(
        data: D,
        reset_wire: WIRE,
        reset_value: [LogicalWireState; W],
    ) -> FlipFlopBus<W, D, impl Bus<W>, impl Bus<W>> {
        let reset_fanout = FanoutBus::new(reset_wire);

        let set_mask = ConstBus::new(reset_value);
        let set_bus = reset_fanout.and(set_mask);

        let reset_mask = ConstBus::new(reset_value.map(|value| !value));
        let reset_bus = reset_fanout.and(reset_mask);

        FlipFlopBus::new(data, reset_bus, set_bus)
    }
}

impl<const W: usize, D: Bus<W>, R: Bus<W>, S: Bus<W>> Bus<W> for FlipFlopBus<W, D, R, S> {
    fn eval(self) -> [WireState; W] {
        self.current_state
    }
}
