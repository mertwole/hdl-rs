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

trait ConcatMarker {}

struct WireConcatMarker;
impl ConcatMarker for WireConcatMarker {}

struct BusConcatMarker;
impl ConcatMarker for BusConcatMarker {}

trait Concat<
    const LEFT_BUS_WIDTH: usize,
    const RIGHT_BUS_WIDTH: usize,
    ML: ConcatMarker,
    MR: ConcatMarker,
> where
    [(); LEFT_BUS_WIDTH + RIGHT_BUS_WIDTH]:,
{
    fn concat(self) -> impl Bus<{ LEFT_BUS_WIDTH + RIGHT_BUS_WIDTH }>;
}

impl<WL: Wire, WR: Wire> Concat<1, 1, WireConcatMarker, WireConcatMarker> for (WL, WR) {
    fn concat(self) -> impl Bus<{ 1 + 1 }> {
        WireAndWire {
            lhs: self.0,
            rhs: self.1,
        }
    }
}

impl<const WL: usize, BL: Bus<WL>, const WR: usize, BR: Bus<WR>>
    Concat<WL, WR, BusConcatMarker, BusConcatMarker> for (BL, BR)
where
    [(); WL + WR]:,
{
    fn concat(self) -> impl Bus<{ WL + WR }> {
        self.0.append_bus_right(self.1)
    }
}

// impl<WIRE: Wire, const W: usize, B: Bus<W>> Concat<1, W, WireConcatMarker, BusConcatMarker>
//     for (WIRE, B)
// where
//     [(); 1 + W]:,
// {
//     fn concat(self) -> impl Bus<{ W + 1 }> {
//         self.1.append_wire_left(self.0)
//     }
// }

// impl<const W: usize, B: Bus<W>, WIRE: Wire> Concat<W, 1, BusConcatMarker, WireConcatMarker>
//     for (B, WIRE)
// where
//     [(); 1 + W]:,
// {
//     fn concat(self) -> impl Bus<{ W + 1 }> {
//         self.0.append_wire_right(self.1)
//     }
// }
