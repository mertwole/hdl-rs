use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use autoimpl_operators::derive_bus_bitwise_ops;

use super::Bus;
use crate::{
    api::{bus::HasBusId, wire_state::WireState},
    intermediate_repr::BusId,
};

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

    fn build_intermediate_repr(
        self,
        _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
    }
}

impl<const W: usize> HasBusId for FeedbackOutput<W> {
    fn get_id(self) -> BusId {
        self.bus_id.expect("Was set in `set_value`")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    #[derive_bus_bitwise_ops(2)]
    struct MockBus([WireState; 2]);

    impl Bus<2> for MockBus {
        const COMBINATIONAL_NETWORK_ID: usize = 2;

        fn eval(self) -> [WireState; 2] {
            self.0
        }

        fn build_intermediate_repr(
            self,
            _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
        ) {
            unimplemented!()
        }
    }

    impl HasBusId for MockBus {
        fn get_id(self) -> BusId {
            BusId::mock()
        }
    }

    #[derive(Clone, Copy)]
    #[derive_bus_bitwise_ops(2)]
    struct MockFeedbackOutputBus {}

    impl Bus<2> for MockFeedbackOutputBus {
        const COMBINATIONAL_NETWORK_ID: usize = 0;

        fn eval(self) -> [WireState; 2] {
            unimplemented!()
        }

        fn build_intermediate_repr(
            self,
            _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
        ) {
            unimplemented!()
        }
    }

    impl HasBusId for MockFeedbackOutputBus {
        fn get_id(self) -> BusId {
            BusId::mock()
        }
    }

    #[test]
    fn test_feedback_evals_correctly() {
        let mut feedback = FeedbackOutput::new();
        assert_eq!(feedback.eval(), [WireState::X; 2]);

        let bus = MockBus([WireState::Zero, WireState::One]);
        feedback.set_value(MockFeedbackOutputBus {}, bus);
        assert_eq!(feedback.eval(), bus.eval())
    }

    #[test]
    fn test_multiple_feedback_buses() {
        let mut feedbacks: Vec<_> = (0..4).map(|_| FeedbackOutput::<2>::new()).collect();

        let mut buses = vec![];
        for a in [WireState::Zero, WireState::One] {
            for b in [WireState::Zero, WireState::One] {
                buses.push(MockBus([a, b]));
            }
        }

        for (bus, feedback) in buses.iter().zip(feedbacks.iter_mut()) {
            feedback.set_value(MockFeedbackOutputBus {}, *bus);
        }

        for (bus, feedback) in buses.iter().zip(feedbacks.iter_mut()) {
            assert_eq!(feedback.eval(), bus.eval());
        }
    }
}
