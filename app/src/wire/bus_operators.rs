use super::{bus::*, *};

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
}

impl<T, const W: usize> BusOps<W> for T where T: Bus<W> {}

#[derive(Clone, Copy)]
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
