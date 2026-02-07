#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use autoimpl_operators::{WireBitwiseOps, derive_bus_bitwise_ops};

mod api;
use api::prelude::*;

fn main() {}

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

#[cfg(test)]
mod tests {
    use super::{InputBusWrapper, InputWireWrapper};
    use crate::api::{bus::mock::*, prelude::*, wire::mock::*};

    #[test]
    fn test_finite_module_instantiation() {
        let a = MockInput::new(WireState::Zero);
        let b = MockInput::new(WireState::One);
        let c = MockInput::new(WireState::Z);

        let _out = module_example(
            InputWireWrapper(a),
            InputWireWrapper(b),
            InputWireWrapper(c),
        );

        let bus_1 = MockInputBus::new([WireState::Zero; 8]);
        let bus_2 = MockInputBus::new([WireState::One; 8]);

        let _out_bus = module_example_with_buses(InputBusWrapper(bus_1), InputBusWrapper(bus_2));

        let a = MockInput::new(WireState::Zero);

        let _out = module_example_with_feedback_wire(InputWireWrapper(a));
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

        super::concat!(a_left, a_middle_inv, a_right, b)
    }

    fn module_example_with_feedback_wire<A: InputWire + 'static>(
        a: InputWireWrapper<A>,
    ) -> impl Wire {
        let feedback = FeedbackWireOutput::new();
        let and = a & feedback;
        let ff = FlipFlop::new(and, ConstZeroWire {}, ConstZeroWire {}, ConstZeroWire {});
        feedback.set_value(ff);

        ff
    }

    fn module_example_with_feedback_bus<A: InputBus<2> + 'static>(
        a: InputBusWrapper<2, A>,
    ) -> impl Bus<2> {
        let feedback = FeedbackBusOutput::new();
        let and = a & feedback;
        let ff = FlipFlopBus::new(
            and,
            ConstZeroWire {},
            ConstBus::new([LogicalWireState::Zero; 2]),
            ConstBus::new([LogicalWireState::Zero; 2]),
        );
        feedback.set_value(ff);

        ff
    }
}
