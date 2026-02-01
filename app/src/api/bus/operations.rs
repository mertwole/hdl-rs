use autoimpl_operators::BitwiseOps;

use crate::api::prelude::*;

pub trait BusOps<const W: usize>: Bus<W> {
    fn wire_at<const I: usize>(self) -> WireAt<W, Self, I>
    where
        [(); W - I - 1]:,
    {
        WireAt { bus: self }
    }

    fn sub_bus<const FROM: usize, const TO: usize>(self) -> SubBus<W, Self, FROM, { TO - FROM }>
    where
        [(); TO - FROM]:,
        [(); W - TO]:,
    {
        SubBus { bus: self }
    }

    fn append_wire_left<WIRE: Wire>(self, wire: WIRE) -> BusWithWireToLeft<W, Self, WIRE> {
        BusWithWireToLeft { bus: self, wire }
    }

    fn append_wire_right<WIRE: Wire>(self, wire: WIRE) -> BusWithWireToRight<W, Self, WIRE> {
        BusWithWireToRight { bus: self, wire }
    }

    fn append_bus_right<const WIDTH: usize, B: Bus<WIDTH>>(
        self,
        bus: B,
    ) -> BusConcat<W, Self, WIDTH, B> {
        BusConcat {
            lhs: self,
            rhs: bus,
        }
    }

    fn and<RB: Bus<W>>(self, rhs: RB) -> BusAnd<W, Self, RB> {
        BusAnd { lhs: self, rhs }
    }
}

impl<T, const W: usize> BusOps<W> for T where T: Bus<W> {}

#[derive(Clone, Copy, BitwiseOps)]
pub struct WireAt<const W: usize, B: Bus<W>, const I: usize> {
    bus: B,
}

impl<const W: usize, B: Bus<W>, const I: usize> Wire for WireAt<W, B, I> {
    fn eval(&self) -> WireState {
        self.bus.eval()[I]
    }
}

#[derive(Clone, Copy)]
pub struct SubBus<const W: usize, B: Bus<W>, const FROM: usize, const WIDTH: usize> {
    bus: B,
}

impl<const W: usize, B: Bus<W>, const FROM: usize, const WIDTH: usize> Bus<WIDTH>
    for SubBus<W, B, FROM, WIDTH>
{
    fn eval(self) -> [WireState; WIDTH] {
        self.bus.eval()[FROM..FROM + WIDTH]
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
pub struct BusWithWireToLeft<const W: usize, B: Bus<W>, WIRE: Wire> {
    bus: B,
    wire: WIRE,
}

impl<const W: usize, B: Bus<W>, WIRE: Wire> Bus<{ W + 1 }> for BusWithWireToLeft<W, B, WIRE> {
    fn eval(self) -> [WireState; W + 1] {
        [&[self.wire.eval()], &self.bus.eval()[..]]
            .concat()
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
pub struct BusWithWireToRight<const W: usize, B: Bus<W>, WIRE: Wire> {
    bus: B,
    wire: WIRE,
}

impl<const W: usize, B: Bus<W>, WIRE: Wire> Bus<{ W + 1 }> for BusWithWireToRight<W, B, WIRE> {
    fn eval(self) -> [WireState; W + 1] {
        [&self.bus.eval()[..], &[self.wire.eval()]]
            .concat()
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
pub struct BusConcat<const W1: usize, B1: Bus<W1>, const W2: usize, B2: Bus<W2>> {
    lhs: B1,
    rhs: B2,
}

impl<const W1: usize, B1: Bus<W1>, const W2: usize, B2: Bus<W2>> Bus<{ W1 + W2 }>
    for BusConcat<W1, B1, W2, B2>
{
    fn eval(self) -> [WireState; W1 + W2] {
        [&self.lhs.eval()[..], &self.rhs.eval()[..]]
            .concat()
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
pub struct BusAnd<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<W> for BusAnd<W, BL, BR> {
    fn eval(self) -> [WireState; W] {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        let result: Vec<_> = (0..W).map(|i| lhs[i].and(rhs[i])).collect();
        result.try_into().expect("Checked to match the length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [WireState; 8] = [
        WireState::Zero,
        WireState::One,
        WireState::X,
        WireState::Z,
        WireState::Zero,
        WireState::One,
        WireState::X,
        WireState::Z,
    ];

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

    #[test]
    fn test_sub_bus() {
        let bus = TestBus(VALUES);

        let sub_bus = bus.sub_bus::<4, 6>();
        assert_eq!(sub_bus.eval(), VALUES[4..6]);

        let sub_bus = bus.sub_bus::<0, 3>();
        assert_eq!(sub_bus.eval(), VALUES[..3]);

        let sub_bus = bus.sub_bus::<3, 8>();
        assert_eq!(sub_bus.eval(), VALUES[3..8]);

        let sub_bus = bus.sub_bus::<1, 1>();
        assert_eq!(sub_bus.eval(), VALUES[1..1]);
    }

    #[test]
    fn test_append_wire_right() {
        let bus = TestBus(VALUES);
        let wire = TestWire(WireState::X);
        let appended = bus.append_wire_right(wire);

        assert_eq!(
            appended.eval().to_vec(),
            [&VALUES[..], &[WireState::X]].concat()
        );
    }

    #[test]
    fn test_append_wire_left() {
        let bus = TestBus(VALUES);
        let wire = TestWire(WireState::X);
        let appended = bus.append_wire_left(wire);

        assert_eq!(
            appended.eval().to_vec(),
            [&[WireState::X], &VALUES[..]].concat()
        );
    }

    #[test]
    fn test_append_bus_right() {
        let bus_2_values = [WireState::X, WireState::X, WireState::X];

        let bus_1 = TestBus(VALUES);
        let bus_2 = TestBus(bus_2_values);
        let appended = bus_1.append_bus_right(bus_2);

        assert_eq!(
            appended.eval().to_vec(),
            [&VALUES[..], &bus_2_values].concat()
        );
    }

    #[test]
    fn test_wire_at() {
        let bus = TestBus(VALUES);

        let wire_4 = bus.wire_at::<4>();
        assert_eq!(wire_4.eval(), WireState::Zero);

        let wire_0 = bus.wire_at::<0>();
        assert_eq!(wire_0.eval(), WireState::Zero);

        let wire_7 = bus.wire_at::<7>();
        assert_eq!(wire_7.eval(), WireState::Z);
    }
}
