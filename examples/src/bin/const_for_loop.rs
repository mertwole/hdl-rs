#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn _const_for_loop(a: impl Bus<8>, b: impl Bus<8>) -> impl Bus<8> {
    const_for_loop!(
        0..8
        {
            let carry = ConstBus::new([LogicalWireState::Zero]);
            let output = ConstBus::new([]);
        }
        {
            let ai = a.wire_at::<{iterator_literal!()}>();
            let bi = b.wire_at::<{iterator_literal!()}>();
            let carry = ai & bi & carry;
            let output = concat_buses!(output, carry);
        }
    );

    output
}

#[test]
fn test_const_for_loop() {
    let a = ConstBus::new([LogicalWireState::Zero; 8]);
    let b = ConstBus::new([LogicalWireState::Zero; 8]);

    let _ = _const_for_loop(a, b);
}
