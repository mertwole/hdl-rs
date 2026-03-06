#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn _feedback_loops<A: InputBus<2> + 'static>(a: InputBusWrapper<2, A>) -> impl Bus<2> {
    let mut feedback = FeedbackOutput::new();
    let and = a & feedback;
    let ff = FlipFlopBus::new(and, ConstBus::new([LogicalWireState::Zero]));
    feedback.set_value(and, ff);

    ff
}

#[test]
fn test_feedback_loops() {
    use hdl_rs::api::testing::*;

    let a = TestInputBus::new([WireState::Zero, WireState::One]);
    let output = _feedback_loops(InputBusWrapper::new(a));

    let _value = output.eval();
}