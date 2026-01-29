mod wire;

use autoimpl_operators::BitwiseOps;
use wire::operators::*;
use wire::*;

fn main() {
    let output = module_example(TestInput::one(), TestInput::one(), TestInput::zero());
    let value = output.eval();

    println!("Value: {:?}", value);
}

#[derive(Clone, Copy, Debug, BitwiseOps)]
struct TestInput {
    state: WireState,
}

#[allow(dead_code)]
impl TestInput {
    fn one() -> Self {
        Self {
            state: WireState::One,
        }
    }

    fn zero() -> Self {
        Self {
            state: WireState::Zero,
        }
    }

    fn x() -> Self {
        Self {
            state: WireState::X,
        }
    }

    fn z() -> Self {
        Self {
            state: WireState::Z,
        }
    }
}

impl Wire for TestInput {
    fn eval(&self) -> WireState {
        self.state
    }
}

impl InputWire for TestInput {}

// TODO: Accept generic struct instead of `impl InputWire` to be able to apply operators to inputs.
fn module_example(a: impl InputWire, b: impl InputWire, c: impl InputWire) -> impl Wire {
    let temp_a = a.and(b);
    let temp_b = a.and(b);

    let temp_c = temp_a & temp_b | temp_a ^ !temp_b;

    temp_c.and(b).or(c).and(c).not()
}
