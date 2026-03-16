#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use hdl_rs::api::prelude::*;

trait StageImpl<
    const Input: usize,
    const Output: usize,
    const StageOutput: usize,
    const Intermediate: usize,
>
{
    fn stage(
        input: impl Bus<Input>,
        intermediate: impl Bus<Intermediate>,
    ) -> (impl Bus<Intermediate>, impl Bus<StageOutput>);
}

struct Adder {}

impl StageImpl<1, 1, 1, 1> for Adder {
    fn stage(input: impl Bus<1>, intermediate: impl Bus<1>) -> (impl Bus<1>, impl Bus<1>) {
        (input, intermediate)
    }
}

fn main() {}

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
