use crate::wire::{
    bus::*,
    bus_operators::{BusOps, BusWithWireToLeft, BusWithWireToRight},
    operators::WireAnd,
    *,
};

#[derive(Clone, Copy)]
struct WireAndWire<WL: Wire, WR: Wire> {
    lhs: WL,
    rhs: WR,
}

impl<WL: Wire, WR: Wire> Bus<2> for WireAndWire<WL, WR> {
    fn eval(self) -> [WireState; 2] {
        [self.lhs.eval(), self.rhs.eval()]
    }
}

struct WireMarker;
struct BusMarker;

trait ConcatenableMarker {}

impl ConcatenableMarker for WireMarker {}
impl ConcatenableMarker for BusMarker {}

trait ConcatWire<WIRE: Wire, const W: usize, M: ConcatenableMarker>
where
    [(); W + 1]:,
{
    fn concat(self, wire: WIRE) -> impl Bus<{ W + 1 }>;
}

impl<const W: usize, B: Bus<W>, WIRE: Wire> ConcatWire<WIRE, W, BusMarker> for B
where
    [(); W + 1]:,
{
    fn concat(self, wire: WIRE) -> impl Bus<{ W + 1 }> {
        self.append_wire_right(wire)
    }
}

impl<WIRE: Wire> ConcatWire<WIRE, 1, WireMarker> for WIRE {
    fn concat(self, wire: WIRE) -> impl Bus<2> {
        WireAndWire {
            lhs: self,
            rhs: wire,
        }
    }
}

trait ConcatBus<const WR: usize, BR: Bus<WR>, const W: usize, M: ConcatenableMarker>
where
    [(); W + WR]:,
{
    fn concat(self, bus: BR) -> impl Bus<{ W + WR }>;
}

impl<const W: usize, B: Bus<W>, const WR: usize, BR: Bus<WR>> ConcatBus<WR, BR, W, BusMarker> for B
where
    [(); W + WR]:,
{
    fn concat(self, bus: BR) -> impl Bus<{ W + WR }> {
        self.append_bus_right(bus)
    }
}

trait ConcatWireAndBus<const WR: usize, BR: Bus<WR>>
where
    [(); WR + 1]:,
{
    fn concat(self, bus: BR) -> impl Bus<{ WR + 1 }>;
}

impl<WIRE: Wire, const W: usize, B: Bus<W>> ConcatWireAndBus<W, B> for WIRE
where
    [(); W + 1]:,
{
    fn concat(self, bus: B) -> impl Bus<{ W + 1 }> {
        bus.append_wire_left(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct TestWire(WireState);

    impl Wire for TestWire {
        fn eval(&self) -> WireState {
            self.0
        }
    }

    #[derive(Clone, Copy)]
    struct TestBus<const W: usize>([WireState; W]);

    impl<const W: usize> Bus<W> for TestBus<W> {
        fn eval(self) -> [WireState; W] {
            self.0
        }
    }

    const BUS_1_VALUES: [WireState; 3] = [WireState::Zero, WireState::Zero, WireState::Zero];
    const BUS_2_VALUES: [WireState; 3] = [WireState::One, WireState::One, WireState::One];
    const WIRE_1_VALUE: WireState = WireState::X;
    const WIRE_2_VALUE: WireState = WireState::Z;

    #[test]
    fn test_concat() {
        let bus_1 = TestBus(BUS_1_VALUES);
        let bus_2 = TestBus(BUS_2_VALUES);
        let wire_1 = TestWire(WIRE_1_VALUE);
        let wire_2 = TestWire(WIRE_2_VALUE);

        let bus_bus = bus_1.concat(bus_2);
        let bus_wire = bus_1.concat(wire_1);
    }
}
