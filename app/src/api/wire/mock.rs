use autoimpl_operators::WireBitwiseOps;

use crate::api::{
    prelude::{Wire, WireState},
    wire::InputWire,
};

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct MockWire(WireState);

impl MockWire {
    pub fn new(value: WireState) -> Self {
        Self(value)
    }
}

impl Wire for MockWire {
    fn eval(&self) -> WireState {
        self.0
    }
}

#[derive(Clone, Copy, Debug, WireBitwiseOps)]
pub struct MockInput {
    state: WireState,
}

impl MockInput {
    pub fn new(state: WireState) -> Self {
        Self { state }
    }
}

impl Wire for MockInput {
    fn eval(&self) -> WireState {
        self.state
    }
}

impl InputWire for MockInput {}
