#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use autoimpl_operators::derive_bus_bitwise_ops;

mod api;
use api::prelude::*;

fn main() {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
struct InputBusWrapper<const W: usize, B: Bus<W>>(B);

impl<const W: usize, B: Bus<W>> Bus<W> for InputBusWrapper<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.0.eval()
    }
}

#[cfg(test)]
mod tests {
    use super::InputBusWrapper;
    use crate::api::{bus::mock::*, prelude::*};

    #[test]
    fn test_finite_module_instantiation() {
        let a = MockInputBus::new([WireState::Zero; 8]);
        let b = MockInputBus::new([WireState::One; 8]);

        let _out = module_example(InputBusWrapper(a), InputBusWrapper(b));

        let a = MockInputBus::new([WireState::Zero; 2]);

        let _out = module_example_with_feedback(InputBusWrapper(a));
    }

    fn module_example<A: InputBus<8>, B: InputBus<8>>(
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

    fn module_example_with_feedback<A: InputBus<2> + 'static>(
        a: InputBusWrapper<2, A>,
    ) -> impl Bus<2> {
        let feedback = FeedbackOutput::new();
        let and = a & feedback;
        let ff = FlipFlopBus::new(
            and,
            ConstBus::new([LogicalWireState::Zero]),
            ConstBus::new([LogicalWireState::Zero; 2]),
            ConstBus::new([LogicalWireState::Zero; 2]),
        );
        feedback.set_value(ff);

        ff
    }
}
