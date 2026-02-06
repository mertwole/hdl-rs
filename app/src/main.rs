#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use autoimpl_operators::{WireBitwiseOps, derive_bus_bitwise_ops};

mod api;
use api::prelude::*;

fn main() {
    let a = TestInput {
        state: WireState::Zero,
    };
    let b = TestInput {
        state: WireState::One,
    };
    let c = TestInput {
        state: WireState::Z,
    };

    let _out = module_example(
        InputWireWrapper(a),
        InputWireWrapper(b),
        InputWireWrapper(c),
    );

    let bus_1 = TestInputBus {
        values: [WireState::Zero; 8],
    };
    let bus_2 = TestInputBus {
        values: [WireState::One; 8],
    };

    let _out_bus = module_example_with_buses(InputBusWrapper(bus_1), InputBusWrapper(bus_2));

    let a = TestInput {
        state: WireState::Zero,
    };
    let b = TestInput {
        state: WireState::One,
    };

    let _out = module_example_with_feedback(InputWireWrapper(a));
}

#[derive(Clone, Copy, Debug, WireBitwiseOps)]
struct TestInput {
    state: WireState,
}

impl Wire for TestInput {
    fn eval(&self) -> WireState {
        self.state
    }
}

impl InputWire for TestInput {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
struct TestInputBus<const W: usize> {
    values: [WireState; W],
}

impl<const W: usize> TestInputBus<W> {
    fn new(values: [WireState; W]) -> Self {
        Self { values }
    }
}

impl<const W: usize> Bus<W> for TestInputBus<W> {
    fn eval(self) -> [WireState; W] {
        self.values
    }
}

impl<const W: usize> InputBus<W> for TestInputBus<W> {}

#[derive(Clone, Copy, WireBitwiseOps)]
struct InputWireWrapper<W: InputWire>(W);

impl<W: InputWire> Wire for InputWireWrapper<W> {
    fn eval(&self) -> WireState {
        self.0.eval()
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
struct InputBusWrapper<const W: usize, B: Bus<W>>(B);

impl<const W: usize, B: Bus<W>> Bus<W> for InputBusWrapper<W, B> {
    fn eval(self) -> [WireState; W] {
        self.0.eval()
    }
}

fn module_example<A: InputWire, B: InputWire, C: InputWire>(
    a: InputWireWrapper<A>,
    b: InputWireWrapper<B>,
    c: InputWireWrapper<C>,
) -> impl Wire {
    let temp_a = a & !b;
    let temp_b = a & b;

    let temp_c = temp_a & temp_b | temp_a ^ !temp_b;

    temp_c.and(b).or(c).and(c).not()
}

fn module_example_with_buses<A: InputBus<8>, B: InputBus<8>>(
    a: InputBusWrapper<8, A>,
    b: InputBusWrapper<8, B>,
) -> impl Bus<16> {
    let c = a & b | a ^ !b;

    let a_left = c.sub_bus::<0, 3>();
    let a_middle = a.wire_at::<3>();
    let a_right = a.sub_bus::<4, 8>();

    let a_middle_inv = !a_middle;

    concat!(a_left, a_middle_inv, a_right, b)
}

fn module_example_with_feedback<A: InputWire + 'static>(a: InputWireWrapper<A>) -> impl Wire {
    let mut feedback = FeedbackWireOutput::new();
    let and = a & feedback;
    let ff = FlipFlop::new(and, ConstZeroWire {}, ConstZeroWire {}, ConstZeroWire {});
    feedback.set_value(ff);

    ff
}
