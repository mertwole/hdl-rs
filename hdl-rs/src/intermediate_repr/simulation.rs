use std::collections::HashMap;

use crate::{
    api::prelude::*,
    intermediate_repr::{BinaryGateOperator, BusId, Gate, IntermediateRepr, UnaryGateOperator},
};

pub enum SimulationEvent {
    Tick,
    CommitStateChanges,
}

pub struct Simulator {
    intermediate_repr: IntermediateRepr,
    inputs: HashMap<BusId, Vec<WireState>>,
    flip_flop_state: HashMap<BusId, FlipFlopState>,
}

struct FlipFlopState {
    clock: WireState,
    current: Vec<WireState>,
    next: Vec<WireState>,
}

impl Simulator {
    pub fn new(intermediate_repr: IntermediateRepr) -> Self {
        Self {
            intermediate_repr,
            inputs: HashMap::new(),
            flip_flop_state: HashMap::new(),
        }
    }

    pub fn set_inputs(&mut self, inputs: HashMap<BusId, Vec<WireState>>) {
        self.inputs = inputs;
    }

    pub fn simulate(&mut self, event: SimulationEvent) {
        for node in self.intermediate_repr.nodes.values() {
            let Gate::FlipFlop(ff) = node else {
                continue;
            };

            let data = self.get_ctx().eval_bus(ff.data);

            let clock = self.get_ctx().eval_bus(ff.clock);
            assert_eq!(clock.len(), 1);
            let clock = clock[0];

            let ff_state = self.flip_flop_state.get_mut(&ff.output);

            match event {
                SimulationEvent::Tick => match ff_state {
                    Some(ff_state) => {
                        if clock == WireState::One && ff_state.clock == WireState::Zero {
                            ff_state.next = data;
                        }
                        ff_state.clock = clock;
                    }
                    None => {
                        self.flip_flop_state.insert(
                            ff.output,
                            FlipFlopState {
                                clock,
                                current: vec![WireState::X; ff.output.width()],
                                next: vec![WireState::X; ff.output.width()],
                            },
                        );
                    }
                },
                SimulationEvent::CommitStateChanges => {
                    if let Some(ff_state) = ff_state {
                        ff_state.current = ff_state.next.clone();
                    }
                }
            }
        }
    }

    fn get_ctx<'a>(&'a self) -> SimulationContext<'a> {
        SimulationContext {
            input_values: &self.inputs,
            intermediate_repr: &self.intermediate_repr,
            flip_flop_state: &self.flip_flop_state,
        }
    }
}

struct SimulationContext<'a> {
    input_values: &'a HashMap<BusId, Vec<WireState>>,
    intermediate_repr: &'a IntermediateRepr,
    flip_flop_state: &'a HashMap<BusId, FlipFlopState>,
}

impl SimulationContext<'_> {
    // TODO: Move out as fn?
    fn eval_bus(&self, bus_id: BusId) -> Vec<WireState> {
        if let Some(value) = self.input_values.get(&bus_id) {
            return value.clone();
        }

        let gate = self
            .intermediate_repr
            .nodes
            .get(&bus_id)
            .expect("Cannot find node with given bus_id");
        eval_gate(gate, self)
    }
}

fn eval_gate(gate: &Gate, ctx: &SimulationContext) -> Vec<WireState> {
    match gate {
        Gate::Input(input) => ctx.eval_bus(input.id),
        Gate::Const(const_bus) => const_bus.value.iter().copied().map(Into::into).collect(),
        Gate::Unary(unary) => {
            let input = ctx.eval_bus(unary.input);

            let result = match unary.operator {
                UnaryGateOperator::Not => input.into_iter().map(|value| value.not()).collect(),
                UnaryGateOperator::ShiftLeft { shift } => (0..unary.output.width())
                    .map(|i| {
                        if i + shift >= unary.output.width() {
                            WireState::Zero
                        } else {
                            input[i + shift]
                        }
                    })
                    .collect(),
                UnaryGateOperator::ShiftRight { shift } => (0..unary.output.width())
                    .map(|i| {
                        if shift > i {
                            WireState::Zero
                        } else {
                            input[i - shift]
                        }
                    })
                    .collect(),
                UnaryGateOperator::SubBus { from, to } => {
                    input[unary.input.width() - 1 - from..unary.input.width() - to].to_vec()
                }
                UnaryGateOperator::Fanout => {
                    vec![input[0]; unary.output.width()]
                }
            };

            assert_eq!(result.len(), unary.output.width());
            result
        }
        Gate::Binary(binary) => {
            let mut lhs = ctx.eval_bus(binary.lhs);
            let mut rhs = ctx.eval_bus(binary.rhs);

            let zipped = lhs.iter().zip(rhs.iter());

            let result: Vec<_> = match binary.operator {
                BinaryGateOperator::And => zipped.map(|(lhs, rhs)| lhs.and(*rhs)).collect(),
                BinaryGateOperator::Or => zipped.map(|(lhs, rhs)| lhs.or(*rhs)).collect(),
                BinaryGateOperator::Xor => zipped.map(|(lhs, rhs)| lhs.xor(*rhs)).collect(),
                BinaryGateOperator::Concat => {
                    lhs.append(&mut rhs);
                    lhs
                }
            };

            assert_eq!(result.len(), binary.output.width());
            result
        }
        Gate::FlipFlop(ff) => match ctx.flip_flop_state.get(&ff.output) {
            Some(state) => state.current.clone(),
            None => vec![WireState::X; ff.output.width()],
        },
    }
}
