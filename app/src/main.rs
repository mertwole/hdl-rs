#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod api;

use api::operators::*;
use api::*;
use autoimpl_operators::BitwiseOps;

use crate::api::{Bus, BusOps, InputBus};

fn main() {}

#[derive(Clone, Copy, Debug, BitwiseOps)]
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

// TODO: Accept generic struct instead of `impl InputWire` to be able to apply operators to inputs.
fn _module_example(a: impl InputWire, b: impl InputWire, c: impl InputWire) -> impl Wire {
    let temp_a = a.and(b);
    let temp_b = a.and(b);

    let temp_c = temp_a & temp_b | temp_a ^ !temp_b;

    temp_c.and(b).or(c).and(c).not()
}

fn _module_example_with_buses(a: impl InputBus<8>, b: impl InputBus<8>) -> impl Bus<16> {
    let a_left = a.sub_bus::<0, 3>();
    let a_middle = a.wire_at::<3>();
    let a_right = a.sub_bus::<4, 8>();

    let a_middle_inv = !a_middle;

    concat!(a_left, a_middle_inv, a_right, b)
}
