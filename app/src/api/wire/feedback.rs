use autoimpl_operators::WireBitwiseOps;

use crate::api::wire::WireNoCopy;

use super::{Wire, WireState};

pub struct FeedbackWireInput {
    wire: Option<Box<dyn WireNoCopy>>,
}

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct FeedbackWireOutput {}

impl FeedbackWireOutput {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_input(self) -> FeedbackWireInput {
        FeedbackWireInput { wire: None }
    }
}

impl Wire for FeedbackWireOutput {
    fn eval(&self) -> super::WireState {
        todo!()
    }
}

impl FeedbackWireInput {
    pub fn set_value<W: WireNoCopy>(&mut self, wire: W) {
        self.wire = Some(Box::from(wire));
    }
}
