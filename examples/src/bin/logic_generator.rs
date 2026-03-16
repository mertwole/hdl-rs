#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

struct StageOutput<const WO: usize, O: Bus<WO>, const WI: usize, I: Bus<WI>> {
    pub output: O,
    pub intermediate: I,
}

trait Generator<
    Input,
    const OUTPUT_WIDTH: usize,
    const STAGE_OUTPUT_WIDTH: usize,
    const INTERMEDIATE_WIDTH: usize,
>
{
    fn stage(
        input: Input,
        intermediate: impl Bus<INTERMEDIATE_WIDTH>,
    ) -> StageOutput<
        STAGE_OUTPUT_WIDTH,
        impl Bus<STAGE_OUTPUT_WIDTH>,
        INTERMEDIATE_WIDTH,
        impl Bus<INTERMEDIATE_WIDTH>,
    >;
}

struct Test {}

struct TestInput<const W: usize, A: Bus<W>, B: Bus<W>> {
    a: A,
    b: B,
}

impl<const W: usize, A: Bus<W>, B: Bus<W>> Generator<TestInput<W, A, B>, W, 1, 1> for Test {
    fn stage(
        input: TestInput<W, A, B>,
        intermediate: impl Bus<1>,
    ) -> StageOutput<1, impl Bus<1>, 1, impl Bus<1>> {
        StageOutput {
            output: intermediate,
            intermediate,
        }
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
