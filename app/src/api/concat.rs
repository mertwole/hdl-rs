use crate::api::{bus::*, bus_operators::BusOps, *};

#[macro_export]
macro_rules! concat {
    ($only_one:expr) => {
        $crate::api::concat::ToBus::to_bus($only_one)
    };
    ($first:expr, $($rest:expr),+) => {
        $crate::api::concat::ToBus::to_bus($first).append_bus_right(concat!($($rest),*))
    };
}

pub trait ToBus<const W: usize, M: ToBusMarker> {
    fn to_bus(self) -> impl Bus<W>;
}

impl<W: Wire> ToBus<1, WireToBusMarker> for W {
    fn to_bus(self) -> impl Bus<1> {
        SingleWireBus { wire: self }
    }
}

impl<const W: usize, B: Bus<W>> ToBus<W, BusToBusMarker> for B {
    fn to_bus(self) -> impl Bus<W> {
        self
    }
}

pub trait ToBusMarker {}

pub struct WireToBusMarker {}
impl ToBusMarker for WireToBusMarker {}

pub struct BusToBusMarker {}
impl ToBusMarker for BusToBusMarker {}

#[derive(Clone, Copy)]
struct SingleWireBus<W: Wire> {
    wire: W,
}

impl<W: Wire> Bus<1> for SingleWireBus<W> {
    fn eval(self) -> [WireState; 1] {
        [self.wire.eval()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Place this and all similar to the `wire` module itself.
    #[derive(Clone, Copy)]
    struct TestWire(WireState);

    impl Wire for TestWire {
        fn eval(&self) -> WireState {
            self.0
        }
    }
    // TODO: Place this and all similar to the `bus` module itself.
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

        let bus_bus = concat!(bus_1, bus_2);
        assert_eq!(
            bus_bus.eval().to_vec(),
            [&BUS_1_VALUES[..], &BUS_2_VALUES[..]].concat()
        );

        let bus_wire = concat!(bus_1, wire_1);
        assert_eq!(
            bus_wire.eval().to_vec(),
            [&BUS_1_VALUES[..], &[WIRE_1_VALUE]].concat()
        );

        let wire_bus = concat!(wire_1, bus_1);
        assert_eq!(
            wire_bus.eval().to_vec(),
            [&[WIRE_1_VALUE], &BUS_1_VALUES[..]].concat()
        );

        let wire_wire = concat!(wire_1, wire_2);
        assert_eq!(wire_wire.eval().to_vec(), [WIRE_1_VALUE, WIRE_2_VALUE]);

        let wire_bus_wire_bus = concat!(wire_1, bus_1, wire_2, bus_2);
        assert_eq!(
            wire_bus_wire_bus.eval().to_vec(),
            [
                &[WIRE_1_VALUE],
                &BUS_1_VALUES[..],
                &[WIRE_2_VALUE],
                &BUS_2_VALUES[..]
            ]
            .concat()
        );
    }
}
