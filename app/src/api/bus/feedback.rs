use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use autoimpl_operators::derive_bus_bitwise_ops;

use super::Bus;
use crate::api::wire_state::WireState;

static FEEDBACK_REGISTRY: OnceLock<FeedbackRegistry> = OnceLock::new();

struct FeedbackRegistry {
    eval_fns: Rc<RefCell<Vec<Option<Box<dyn Fn() -> Vec<WireState>>>>>>,
}

unsafe impl Send for FeedbackRegistry {}
unsafe impl Sync for FeedbackRegistry {}

impl FeedbackRegistry {
    fn new() -> Self {
        Self {
            eval_fns: Default::default(),
        }
    }

    fn allocate_id(&self) -> usize {
        self.eval_fns.borrow_mut().push(None);
        self.eval_fns.borrow().len() - 1
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FeedbackOutput<const W: usize> {
    id: usize,
}

impl<const W: usize> FeedbackOutput<W> {
    pub fn new() -> Self {
        let id = FEEDBACK_REGISTRY
            .get_or_init(FeedbackRegistry::new)
            .allocate_id();

        Self { id }
    }

    pub fn set_value<B: Bus<W> + 'static>(&self, bus: B) {
        let registry = FEEDBACK_REGISTRY.get().expect(
            "The FeedbackWireOutput is created in the `new` so OnceLock must be initialized at this point",
        );
        let eval = Box::from(move || bus.eval().to_vec());
        registry.eval_fns.borrow_mut()[self.id] = Some(eval);
    }
}

impl<const W: usize> Bus<W> for FeedbackOutput<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn eval(self) -> [WireState; W] {
        let registry = FEEDBACK_REGISTRY.get().expect(
            "The FeedbackWireOutput is created in the `new` so OnceLock must be initialized at this point",
        );
        let eval_fns = &registry.eval_fns.borrow()[..];
        let eval = eval_fns[self.id]
            .as_ref()
            .expect("TODO: Restrict not using the set_value");

        eval().try_into().expect("Checked to match the width")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::bus::mock::*;

    #[test]
    fn test_feedback_evals_correctly() {
        let feedback = FeedbackOutput::new();
        let bus = MockBus::new([WireState::One, WireState::Zero]);
        feedback.set_value(bus);
        assert_eq!(feedback.eval(), [WireState::One, WireState::Zero])
    }
}
