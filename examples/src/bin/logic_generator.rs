#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn adder(a: impl Bus<8>, b: impl Bus<8>) -> impl Bus<8> {
    const_for_loop!(
        0..8
        {
            let carry = ConstBus::new([LogicalWireState::Zero]);
            let output = ConstBus::new([]);
        }
        {
            let c = adder_stage::<{iterator_literal!()}>(a, b);

            let carry = c.and(carry);

            let out = carry.and(c);

            let output = concat_buses!(output, out);
        }
    );

    output
}

fn adder_stage<const I: usize>(a: impl Bus<8>, b: impl Bus<8>) -> impl Bus<1>
where
    [(); 8 - I - 1]:,
{
    let ai = a.wire_at::<I>();
    let bi = b.wire_at::<I>();

    ai.and(bi)
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
