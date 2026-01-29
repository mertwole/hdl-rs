mod wire;

use wire::*;

fn main() {
    let output = module_example(TestInput::one(), TestInput::one(), TestInput::zero());
    let value = output.eval();

    println!("Value: {:?}", value);
}

#[derive(Clone, Copy, Debug)]
struct TestInput {
    state: WireState,
}

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

fn module_example(a: impl InputWire, b: impl InputWire, c: impl InputWire) -> impl Wire {
    a.and(b).or(c).and(c).not()
}
