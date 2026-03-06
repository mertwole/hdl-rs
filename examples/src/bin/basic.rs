#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn _basic<A: InputBus<8> + ClockBus, B: InputBus<8>>(
    a: InputBusWrapper<8, A>,
    b: InputBusWrapper<8, B>,
) -> impl Bus<16> {
    let c = a & b | a ^ !b;

    let c_left = c.sub_bus::<3, 1>();
    let a_middle = a.wire_at::<3>();
    let a_right = a.sub_bus::<7, 4>();

    let ff = FlipFlopBus::new(c_left, a_middle);

    let a_middle_inv = !a_middle;

    concat_buses!(ff, a_middle_inv, a_right, b)
}

#[test]
fn test_basic() {
    use hdl_rs::api::testing::*;

    let a = TestInputBus::new([WireState::One; 8]);
    let b = TestInputBus::new([WireState::Zero; 8]);

    let output = _basic(InputBusWrapper::new(a), InputBusWrapper::new(b));

    let _value = output.eval();
}
