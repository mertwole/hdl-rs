use derive_macros::derive_bus_bitwise_ops;

use super::Bus;
use crate::intermediate_repr::BusId;

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
