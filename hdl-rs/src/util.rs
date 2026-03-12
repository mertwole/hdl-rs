use derive_macros::logic_generator;

use crate::api::{
    bus::{Bus, BusOps, ConstBus},
    prelude::LogicalWireState,
};

fn adder_stage<const I: usize>(
    input: impl Bus<1>,
    a: impl Bus<8>,
    b: impl Bus<8>,
) -> (impl Bus<1>, impl Bus<1>)
where
    [(); 8 - I - 1]:,
{
    let ai = a.wire_at::<I>();
    let bi = b.wire_at::<I>();

    let and = ai.and(bi);

    (and, and)
}

#[test]
fn aaa() {
    let a = ConstBus::new([LogicalWireState::Zero; 8]);
    let b = ConstBus::new([LogicalWireState::Zero; 8]);

    let input = ConstBus::new([LogicalWireState::Zero]);

    let (x, y) = adder_stage::<3>(input, a, b);
}

#[test]
fn bbb() {
    let a = ConstBus::new([LogicalWireState::Zero; 8]);
    let b = ConstBus::new([LogicalWireState::Zero; 8]);

    let input = ConstBus::new([LogicalWireState::Zero]);

    #[logic_generator(for I in 0..8)]
    {
        let carry = _prev_block_output;

        let (x, y) = adder_stage::<I>(carry, a, b);

        x
    }
}

#[generator_todo]
fn adder_stage_mod<const I: usize>(
    input: impl Bus<1>,
    a: impl Bus<8>,
    b: impl Bus<8>,
) -> impl Bus<1>
where
    [(); 8 - I - 1]:,
{
    let ai = a.wire_at::<I>();
    let bi = b.wire_at::<I>();

    let and = ai.and(bi);

    and
}
