use derive_macros::derive_bus_bitwise_ops;

use super::Bus;
use crate::{intermediate_repr::BusId};

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
pub struct FeedbackOutput<const W: usize> {
    bus_id: Option<BusId>,
}

impl<const W: usize> FeedbackOutput<W> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { bus_id: None }
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
    }
}

impl<const W: usize> Bus<W> for FeedbackOutput<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn get_id(self) -> BusId {
        self.bus_id.expect("Was set in `set_value`")
    }

    fn build_intermediate_repr(
        self,
        _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
    }
}

// TODO: Reintroduce
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Clone, Copy)]
//     #[derive_bus_bitwise_ops(2)]
//     struct MockBus([WireState; 2]);

//     impl Bus<2> for MockBus {
//         const COMBINATIONAL_NETWORK_ID: usize = 2;

//         fn get_id(self) -> BusId {
//             BusId::mock()
//         }

//         fn build_intermediate_repr(
//             self,
//             _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
//         ) {
//             unimplemented!()
//         }
//     }

//     #[derive(Clone, Copy)]
//     #[derive_bus_bitwise_ops(2)]
//     struct MockFeedbackOutputBus {}

//     impl Bus<2> for MockFeedbackOutputBus {
//         const COMBINATIONAL_NETWORK_ID: usize = 0;

//         fn get_id(self) -> BusId {
//             BusId::mock()
//         }

//         fn build_intermediate_repr(
//             self,
//             _builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
//         ) {
//             unimplemented!()
//         }
//     }

//     #[test]
//     fn test_feedback_evals_correctly() {
//         let mut feedback = FeedbackOutput::new();
//         assert_eq!(feedback.eval(), [WireState::X; 2]);

//         let bus = MockBus([WireState::Zero, WireState::One]);
//         feedback.set_value(MockFeedbackOutputBus {}, bus);
//         assert_eq!(feedback.eval(), bus.eval())
//     }

//     #[test]
//     fn test_multiple_feedback_buses() {
//         let mut feedbacks: Vec<_> = (0..4).map(|_| FeedbackOutput::<2>::new()).collect();

//         let mut buses = vec![];
//         for a in [WireState::Zero, WireState::One] {
//             for b in [WireState::Zero, WireState::One] {
//                 buses.push(MockBus([a, b]));
//             }
//         }

//         for (bus, feedback) in buses.iter().zip(feedbacks.iter_mut()) {
//             feedback.set_value(MockFeedbackOutputBus {}, *bus);
//         }

//         for (bus, feedback) in buses.iter().zip(feedbacks.iter_mut()) {
//             assert_eq!(feedback.eval(), bus.eval());
//         }
//     }
// }
