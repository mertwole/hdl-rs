use crate::api::prelude::{Bus, WireState};

#[derive(Clone, Copy)]
pub struct MockBus<const W: usize>([WireState; W]);

impl<const W: usize> MockBus<W> {
    pub fn new(values: [WireState; W]) -> Self {
        Self(values)
    }
}

impl<const W: usize> Bus<W> for MockBus<W> {
    fn eval(self) -> [WireState; W] {
        self.0
    }
}
