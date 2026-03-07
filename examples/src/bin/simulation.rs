#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use hdl_rs::api::prelude::*;

fn main() {}

fn _simulation<A: InputBus<1> + ClockBus, B: InputBus<2>>(
    clk: InputBusWrapper<1, A>,
    data: InputBusWrapper<2, B>,
) -> impl Bus<2> {
    FlipFlopBus::new(data, clk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdl_rs::{
        api::testing::*,
        intermediate_repr::{
            IntermediateReprBuilder,
            simulation::{SimulationEvent, Simulator},
        },
    };

    #[test]
    fn test_simulation() {
        let clk = TestInputBus::new([WireState::One]);
        let data = TestInputBus::new([WireState::One; 2]);

        let output_bus = _simulation(InputBusWrapper::new(clk), InputBusWrapper::new(data));

        let mut builder = IntermediateReprBuilder::new();
        output_bus.build_intermediate_repr(&mut builder);
        builder.push_output(output_bus.get_id());
        let intermediate = builder.build();

        let mut simulator = Simulator::new(intermediate);

        let mut simulate_step =
            |clk_value: WireState, data_value: [WireState; 2], extected_output: [WireState; 2]| {
                simulator.set_inputs(
                    vec![
                        (clk.get_id(), vec![clk_value]),
                        (data.get_id(), data_value.to_vec()),
                    ]
                    .into_iter()
                    .collect(),
                );
                simulator.simulate(SimulationEvent::Tick);
                simulator.simulate(SimulationEvent::CommitStateChanges);

                let outputs = simulator.get_outputs();
                assert_eq!(
                    outputs.get(&output_bus.get_id()),
                    Some(&extected_output.to_vec())
                );
            };

        simulate_step(
            WireState::Zero,
            [WireState::Zero, WireState::One],
            [WireState::X, WireState::X],
        );

        simulate_step(
            WireState::Zero,
            [WireState::One, WireState::One],
            [WireState::X, WireState::X],
        );

        simulate_step(
            WireState::One,
            [WireState::Zero, WireState::Zero],
            [WireState::Zero, WireState::Zero],
        );
    }
}
