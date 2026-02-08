#[macro_export]
macro_rules! concat {
    ($only_one:expr) => {
        $only_one
    };
    ($first:expr, $($rest:expr),+) => {
        $crate::api::bus::BusOps::append($first, ($crate::concat!($($rest),*)))
    };
}

#[cfg(test)]
mod tests {
    use crate::api::{
        bus::{Bus, mock::*},
        wire_state::WireState,
    };

    const BUS_1_VALUES: [WireState; 3] = [WireState::Zero, WireState::Zero, WireState::Zero];
    const BUS_2_VALUES: [WireState; 3] = [WireState::One, WireState::One, WireState::One];
    const BUS_3_VALUES: [WireState; 3] = [WireState::X, WireState::X, WireState::X];

    #[test]
    fn test_concat() {
        let bus_1 = MockBus::new(BUS_1_VALUES);
        let bus_2 = MockBus::new(BUS_2_VALUES);
        let bus_3 = MockBus::new(BUS_3_VALUES);

        let bus_bus = concat!(bus_1, bus_2);
        assert_eq!(
            bus_bus.eval().to_vec(),
            [&BUS_1_VALUES[..], &BUS_2_VALUES[..]].concat()
        );

        let bus_bus_bus = concat!(bus_1, bus_2, bus_3);
        assert_eq!(
            bus_bus_bus.eval().to_vec(),
            [&BUS_1_VALUES[..], &BUS_2_VALUES[..], &BUS_3_VALUES[..]].concat()
        );
    }
}
