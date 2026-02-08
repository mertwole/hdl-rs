use autoimpl_operators::derive_bus_bitwise_ops;

use crate::api::prelude::*;

pub trait BusOps<const W: usize>: Bus<W> {
    fn wire_at<const I: usize>(self) -> SubBus<W, Self, I, 1> {
        SubBus { bus: self }
    }

    fn sub_bus<const FROM: usize, const TO: usize>(self) -> SubBus<W, Self, FROM, { TO - FROM }>
    where
        [(); TO - FROM]:,
        [(); W - TO]:,
    {
        SubBus { bus: self }
    }

    fn append<const WIDTH: usize, B: Bus<WIDTH>>(self, bus: B) -> BusConcat<W, Self, WIDTH, B> {
        BusConcat {
            lhs: self,
            rhs: bus,
        }
    }

    fn and<RB: Bus<W>>(self, rhs: RB) -> BusAnd<W, Self, RB> {
        BusAnd { lhs: self, rhs }
    }

    fn or<RB: Bus<W>>(self, rhs: RB) -> BusOr<W, Self, RB> {
        BusOr { lhs: self, rhs }
    }

    fn xor<RB: Bus<W>>(self, rhs: RB) -> BusXor<W, Self, RB> {
        BusXor { lhs: self, rhs }
    }

    fn not(self) -> BusNot<W, Self> {
        BusNot { bus: self }
    }

    fn lshift<const S: usize>(self) -> BusShiftLeft<W, Self, S> {
        BusShiftLeft { bus: self }
    }

    fn rshift<const S: usize>(self) -> BusShiftRight<W, Self, S> {
        BusShiftRight { bus: self }
    }
}

impl<T, const W: usize> BusOps<W> for T where T: Bus<W> {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(WIDTH)]
pub struct SubBus<const W: usize, B: Bus<W>, const FROM: usize, const WIDTH: usize> {
    bus: B,
}

impl<const W: usize, B: Bus<W>, const FROM: usize, const WIDTH: usize> Bus<WIDTH>
    for SubBus<W, B, FROM, WIDTH>
{
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; WIDTH] {
        self.bus.eval()[FROM..FROM + WIDTH]
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W1 + W2)]
pub struct BusConcat<const W1: usize, B1: Bus<W1>, const W2: usize, B2: Bus<W2>> {
    lhs: B1,
    rhs: B2,
}

impl<const W1: usize, B1: Bus<W1>, const W2: usize, B2: Bus<W2>> Bus<{ W1 + W2 }>
    for BusConcat<W1, B1, W2, B2>
{
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(B1::COMBINATIONAL_NETWORK_ID, B2::COMBINATIONAL_NETWORK_ID);

    fn eval(self) -> [WireState; W1 + W2] {
        [&self.lhs.eval()[..], &self.rhs.eval()[..]]
            .concat()
            .try_into()
            .expect("Checked to match the width")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusAnd<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<W> for BusAnd<W, BL, BR> {
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn eval(self) -> [WireState; W] {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        let result: Vec<_> = (0..W).map(|i| lhs[i].and(rhs[i])).collect();
        result.try_into().expect("Checked to match the length")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusOr<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<W> for BusOr<W, BL, BR> {
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn eval(self) -> [WireState; W] {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        let result: Vec<_> = (0..W).map(|i| lhs[i].or(rhs[i])).collect();
        result.try_into().expect("Checked to match the length")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusXor<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<W> for BusXor<W, BL, BR> {
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn eval(self) -> [WireState; W] {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        let result: Vec<_> = (0..W).map(|i| lhs[i].or(rhs[i])).collect();
        result.try_into().expect("Checked to match the length")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusNot<const W: usize, B: Bus<W>> {
    bus: B,
}

impl<const W: usize, B: Bus<W>> Bus<W> for BusNot<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; W] {
        self.bus.eval().map(|value| value.not())
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusShiftRight<const W: usize, B: Bus<W>, const S: usize> {
    bus: B,
}

impl<const W: usize, B: Bus<W>, const S: usize> Bus<W> for BusShiftRight<W, B, S> {
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; W] {
        let value = self.bus.eval();

        let result: Vec<_> = (0..W)
            .map(|i| if S > i { WireState::Zero } else { value[i - S] })
            .collect();
        result.try_into().expect("Checked to match the length")
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct BusShiftLeft<const W: usize, B: Bus<W>, const S: usize> {
    bus: B,
}

impl<const W: usize, B: Bus<W>, const S: usize> Bus<W> for BusShiftLeft<W, B, S> {
    const COMBINATIONAL_NETWORK_ID: usize = B::COMBINATIONAL_NETWORK_ID;

    fn eval(self) -> [WireState; W] {
        let value = self.bus.eval();

        let result: Vec<_> = (0..W)
            .map(|i| {
                if i + S >= W {
                    WireState::Zero
                } else {
                    value[i + S]
                }
            })
            .collect();
        result.try_into().expect("Checked to match the length")
    }
}

const fn usize_min(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs { lhs } else { rhs }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::bus::mock::*;

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

    #[test]
    fn test_sub_bus() {
        let bus = MockBus::new(VALUES);

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
    fn test_append_bus_right() {
        let bus_2_values = [WireState::X, WireState::X, WireState::X];

        let bus_1 = MockBus::new(VALUES);
        let bus_2 = MockBus::new(bus_2_values);
        let appended = bus_1.append(bus_2);

        assert_eq!(
            appended.eval().to_vec(),
            [&VALUES[..], &bus_2_values].concat()
        );
    }

    #[test]
    fn test_wire_at() {
        let bus = MockBus::new(VALUES);

        let wire_4 = bus.wire_at::<4>();
        assert_eq!(wire_4.eval(), [WireState::Zero]);

        let wire_0 = bus.wire_at::<0>();
        assert_eq!(wire_0.eval(), [WireState::Zero]);

        let wire_7 = bus.wire_at::<7>();
        assert_eq!(wire_7.eval(), [WireState::Z]);
    }
}
