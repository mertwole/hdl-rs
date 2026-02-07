use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use autoimpl_operators::WireBitwiseOps;

use super::{Wire, WireState};

static FEEDBACK_REGISTRY: OnceLock<FeedbackRegistry> = OnceLock::new();

struct FeedbackRegistry {
    eval_fns: Rc<RefCell<Vec<Option<Box<dyn Fn() -> WireState>>>>>,
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

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct FeedbackWireOutput {
    id: usize,
}

impl FeedbackWireOutput {
    pub fn new() -> Self {
        let id = FEEDBACK_REGISTRY
            .get_or_init(FeedbackRegistry::new)
            .allocate_id();

        Self { id }
    }

    pub fn set_value<W: Wire + 'static>(&self, wire: W) {
        let registry = FEEDBACK_REGISTRY.get().expect(
            "The FeedbackWireOutput is created in the `new` so OnceLock must be initialized at this point",
        );
        let eval = Box::from(move || wire.eval());
        registry.eval_fns.borrow_mut()[self.id] = Some(eval);
    }
}

impl Wire for FeedbackWireOutput {
    fn eval(&self) -> super::WireState {
        let registry = FEEDBACK_REGISTRY.get().expect(
            "The FeedbackWireOutput is created in the `new` so OnceLock must be initialized at this point",
        );
        let eval_fns = &registry.eval_fns.borrow()[..];
        let eval = eval_fns[self.id]
            .as_ref()
            .expect("TODO: Restrict not using the set_value");

        eval()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wire::mock::*;

    #[test]
    fn test_feedback_evals_correctly() {
        let feedback = FeedbackWireOutput::new();
        let wire = MockWire::new(WireState::One);
        feedback.set_value(wire);
        assert_eq!(feedback.eval(), WireState::One)
    }
}
