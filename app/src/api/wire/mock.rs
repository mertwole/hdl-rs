use crate::api::prelude::{Wire, WireState};

#[derive(Clone, Copy)]
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
