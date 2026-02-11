use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use autoimpl_operators::derive_bus_bitwise_ops;

use super::Bus;
use crate::{api::wire_state::WireState, intermediate_repr::BusId};

static FEEDBACK_REGISTRY: OnceLock<FeedbackRegistry> = OnceLock::new();

type EvalFn = Box<dyn Fn() -> Vec<WireState>>;

struct FeedbackRegistry {
    eval_fns: Rc<RefCell<Vec<Option<EvalFn>>>>,
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
    bus_id: Option<BusId>,
}

impl<const W: usize> FeedbackOutput<W> {
    pub fn new() -> Self {
        let id = FEEDBACK_REGISTRY
            .get_or_init(FeedbackRegistry::new)
            .allocate_id();

        Self { id, bus_id: None }
    }

    /// `_connected_to`: Bus which this `FeedbackOutput` is connected to as an input.
    pub fn set_value<const WIDTH: usize, BC: Bus<WIDTH>, BI: Bus<W> + 'static>(
        &mut self,
        _connected_to: BC,
        input: BI,
    ) {
        assert!(
            BC::COMBINATIONAL_NETWORK_ID < BI::COMBINATIONAL_NETWORK_ID,
            "Feedback loop is impossible: input and output are in the same combinatorial network"
        );

        self.bus_id = Some(input.get_id());

        let registry = FEEDBACK_REGISTRY.get().expect(
            "The FeedbackWireOutput is created in the `new` so OnceLock must be initialized at this point",
        );
        let eval = Box::from(move || input.eval().to_vec());
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
        eval_fns[self.id]
            .as_ref()
            .map(|eval| eval().try_into().expect("Checked to match the width"))
            .unwrap_or_else(|| [WireState::X; W])
    }

    fn get_id(self) -> BusId {
        self.bus_id.expect("Was set in `set_value`")
    }

    fn build_intermediate_repr(
        self,
        _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct MockBus {}

    impl Bus<2> for MockBus {
        const COMBINATIONAL_NETWORK_ID: usize = 2;

        fn eval(self) -> [WireState; 2] {
            [WireState::Zero, WireState::One]
        }

        fn get_id(self) -> BusId {
            BusId::mock()
        }

        fn build_intermediate_repr(
            self,
            _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
        ) {
            unimplemented!()
        }
    }

    #[derive(Clone, Copy)]
    struct MockFeedbackOutputBus {}

    impl Bus<2> for MockFeedbackOutputBus {
        const COMBINATIONAL_NETWORK_ID: usize = 0;

        fn eval(self) -> [WireState; 2] {
            unimplemented!()
        }

        fn get_id(self) -> BusId {
            BusId::mock()
        }

        fn build_intermediate_repr(
            self,
            _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
        ) {
            unimplemented!()
        }
    }

    #[test]
    fn test_feedback_evals_correctly() {
        let mut feedback = FeedbackOutput::new();
        let bus = MockBus {};
        feedback.set_value(MockFeedbackOutputBus {}, bus);
        assert_eq!(feedback.eval(), bus.eval())
    }
}
