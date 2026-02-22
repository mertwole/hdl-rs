#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(iter_intersperse)]
#![allow(dead_code)]

use autoimpl_operators::{derive_bus_bitwise_ops, derive_clock_bus, derive_reset_bus};

mod api;
use api::prelude::*;

use crate::intermediate_repr::BusId;

mod intermediate_repr;
mod verilog;

fn main() {
    let a = InputBusImpl::new([WireState::Zero; 8]);
    let b = InputBusImpl::new([WireState::One; 8]);

    let out = module_example(InputBusWrapper::new(a), InputBusWrapper::new(b));

    let mut builder = intermediate_repr::IntermediateReprBuilder::new();
    out.build_intermediate_repr(&mut builder);
    builder.push_output(out.get_id());
    let verilog_mod = builder.to_verilog();

    let verilog = verilog_mod.generate_verilog();

    println!("{verilog}");
}

fn module_example<A: InputBus<8> + ClockBus + ResetBus, B: InputBus<8>>(
    a: InputBusWrapper<8, A>,
    b: InputBusWrapper<8, B>,
) -> impl Bus<16> {
    let c = a & b | a ^ !b;

    let a_left = c.sub_bus::<0, 3>();
    let a_middle = a.wire_at::<3>();
    let a_right = a.sub_bus::<4, 8>();

    let ff = FlipFlopBus::new(a_left, a_middle);

    let a_middle_inv = !a_middle;

    concat!(ff, a_middle_inv, a_right, b)
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
#[derive_reset_bus]
struct InputBusImpl<const W: usize> {
    value: [WireState; W],
    id: BusId,
}

impl<const W: usize> InputBusImpl<W> {
    fn new(value: [WireState; W]) -> Self {
        Self {
            value,
            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize> InputBus<W> for InputBusImpl<W> {}

impl<const W: usize> Bus<W> for InputBusImpl<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.value
    }

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(self, builder: &mut intermediate_repr::IntermediateReprBuilder) {
        builder.push_element(intermediate_repr::Gate::Input(
            intermediate_repr::InputBus { id: self.id },
        ));
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus(B)]
#[derive_reset_bus(B)]
struct InputBusWrapper<const W: usize, B: Bus<W>> {
    bus: B,
    id: BusId,
}

impl<const W: usize, B: Bus<W>> InputBusWrapper<W, B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus,
            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize, B: Bus<W>> Bus<W> for InputBusWrapper<W, B> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        self.bus.eval()
    }

    fn get_id(self) -> BusId {
        self.bus.get_id()
    }

    fn build_intermediate_repr(self, builder: &mut intermediate_repr::IntermediateReprBuilder) {
        self.bus.build_intermediate_repr(builder);
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

        let _out = module_example(InputBusWrapper::new(a), InputBusWrapper::new(b));

        let a = MockInputBus::new([WireState::Zero; 2]);

        let _out = module_example_with_feedback(InputBusWrapper::new(a));
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
        let mut feedback = FeedbackOutput::new();
        let and = a & feedback;
        let ff = FlipFlopBus::new(and, ConstBus::new([LogicalWireState::Zero]));
        feedback.set_value(and, ff);

        ff
    }
}
