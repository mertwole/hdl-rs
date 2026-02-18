use crate::{intermediate_repr::IntermediateRepr, verilog};

use super::BusId;

pub struct FlipFlop {
    pub data: BusId,
    // TODO: Process `reset` and `set`.
    pub reset: BusId,
    pub set: BusId,

    pub clock: BusId,

    pub output: BusId,
}

impl IntermediateRepr for FlipFlop {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_register(verilog::RegisterDefinition {
            name: self.output.to_string(),
            width: self.output.width(),
            clock: self.clock.to_string(),
            data_bus: self.data.to_string(),
        });
    }
}
