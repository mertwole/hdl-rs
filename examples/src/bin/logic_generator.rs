#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn adder(a: impl Bus<8>, b: impl Bus<8>) -> impl Bus<8> {
    const_for_loop!(
        for I in 0..8
        let carry = ConstBus::new([LogicalWireState::Zero]);
        // In future: reduction op.
    {
        let ai = a.wire_at::<I>();
        let bi = b.wire_at::<I>();

        let carry = ai.and(bi).and(carry);

        let out = carry.and(ai);

        out
    })
}

#[logic_generator(for I in 0..8)]
fn _adder_stage<const I: usize>(
    #[input] carry: impl Bus<1>,
    a: impl Bus<8>,
    b: impl Bus<8>,
) -> impl Bus<1>
where
    [(); 8 - I - 1]:,
{
    let ai = a.wire_at::<I>();
    let bi = b.wire_at::<I>();

    ai.and(bi).and(carry)
}

#[test]
fn test_adder_stage() {
    let a = ConstBus::new([LogicalWireState::Zero; 8]);
    let b = ConstBus::new([LogicalWireState::Zero; 8]);
    let input = ConstBus::new([LogicalWireState::Zero]);

    let _ = _adder_stage(input, a, b);
}
