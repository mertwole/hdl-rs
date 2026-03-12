use derive_macros::logic_generator;

use crate::api::{
    bus::{Bus, BusOps, ConstBus},
    prelude::LogicalWireState,
};

#[test]
fn test_adder_stage() {
    let a = ConstBus::new([LogicalWireState::Zero; 8]);
    let b = ConstBus::new([LogicalWireState::Zero; 8]);
    let input = ConstBus::new([LogicalWireState::Zero]);

    let _ = adder_stage(input, a, b);
}

#[logic_generator(for I in 0..1)]
fn adder_stage(input: impl Bus<1>, a: impl Bus<8>, b: impl Bus<8>) -> impl Bus<1> {
    let ai = a.wire_at::<I>();
    let bi = b.wire_at::<I>();

    let and = ai.and(bi).and(input);

    and
}
