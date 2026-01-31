use crate::wire::{
    self, Wire, WireState,
    bus::{self, Bus},
    bus_operators::{BusConcat, BusOps, BusWithWireToLeft, BusWithWireToRight},
};

pub trait Concatenable<W: Wire, const WIDTH: usize, B: Bus<WIDTH>>: Clone + Copy {
    fn resolve(self) -> ConcatenableEntity<W, WIDTH, B>;

    fn concat<WR: Wire, const RHS_WIDTH: usize, BR: Bus<RHS_WIDTH>>(
        self,
        rhs: impl Concatenable<WR, RHS_WIDTH, BR>,
    ) -> ConcatentationResult<W, WR, WIDTH, RHS_WIDTH, B, BR>
    where
        [(); WIDTH + RHS_WIDTH]:,
    {
        self.resolve().concat(rhs.resolve())
    }
}

impl<const WIDTH: usize, B: Bus<WIDTH>> Concatenable<wire::Dummy, WIDTH, B> for B {
    fn resolve(self) -> ConcatenableEntity<wire::Dummy, WIDTH, B> {
        ConcatenableEntity::Bus(self)
    }
}

enum ConcatenableEntity<W: Wire, const WIDTH: usize, B: Bus<WIDTH>> {
    Wire(W),
    Bus(B),
}

impl<W: Wire, const WIDTH: usize, B: Bus<WIDTH>> ConcatenableEntity<W, WIDTH, B> {
    fn concat<WR: Wire, const RHS_WIDTH: usize, BR: Bus<RHS_WIDTH>>(
        self,
        rhs: ConcatenableEntity<WR, RHS_WIDTH, BR>,
    ) -> ConcatentationResult<W, WR, WIDTH, RHS_WIDTH, B, BR>
    where
        [(); WIDTH + RHS_WIDTH]:,
    {
        match (self, rhs) {
            (ConcatenableEntity::Wire(wire), ConcatenableEntity::Bus(bus)) => {
                ConcatentationResult::WireAndBus(bus.append_wire_left(wire))
            }
            (ConcatenableEntity::Bus(bus), ConcatenableEntity::Wire(wire)) => {
                ConcatentationResult::BusAndWire(bus.append_wire_right(wire))
            }
            (ConcatenableEntity::Bus(bus_1), ConcatenableEntity::Bus(bus_2)) => {
                ConcatentationResult::BusAndBus(bus_1.append_bus_right(bus_2))
            }
            (ConcatenableEntity::Wire(wire_1), ConcatenableEntity::Wire(wire_2)) => {
                ConcatentationResult::WireAndWire(WireConcat { wire_1, wire_2 })
            }
        }
    }
}

enum ConcatentationResult<
    W1: Wire,
    W2: Wire,
    const WIDTH_1: usize,
    const WIDTH_2: usize,
    B1: Bus<WIDTH_1>,
    B2: Bus<WIDTH_2>,
> {
    WireAndBus(BusWithWireToLeft<WIDTH_2, B2, W1>),
    BusAndWire(BusWithWireToRight<WIDTH_1, B1, W2>),
    BusAndBus(BusConcat<WIDTH_1, B1, WIDTH_2, B2>),
    WireAndWire(WireConcat<W1, W2>),
}

#[derive(Clone, Copy)]
pub struct WireConcat<W1: Wire, W2: Wire> {
    wire_1: W1,
    wire_2: W2,
}

impl<W1: Wire, W2: Wire> Bus<2> for WireConcat<W1, W2> {
    fn eval(self) -> [WireState; 2] {
        [self.wire_1.eval(), self.wire_2.eval()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct TestBus<const W: usize>([WireState; W]);

    impl<const W: usize> Bus<W> for TestBus<W> {
        fn eval(self) -> [WireState; W] {
            self.0
        }
    }

    #[derive(Clone, Copy)]
    struct TestWire(WireState);

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

        bus_1.concat(bus_2);
    }
}
