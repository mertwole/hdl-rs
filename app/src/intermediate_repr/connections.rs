use crate::{
    intermediate_repr::{BusId, IntermediateRepr},
    verilog,
};

pub struct SubBus {
    pub input: BusId,
    pub from: usize,
    pub to: usize,
    pub output: BusId,
}

impl IntermediateRepr for SubBus {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.output.width(),
            // TODO: Properly convert range to the verilog indexes.
            assignment: Some(verilog::Expression::Range {
                wire: self.input.to_string(),
                from: self.from,
                to: self.to,
            }),
        });
    }
}

pub struct BusConcat {
    pub lhs: BusId,
    pub rhs: BusId,
    pub output: BusId,
}

impl IntermediateRepr for BusConcat {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.output.width(),
            assignment: Some(verilog::Expression::Concat {
                lhs: self.lhs.to_string(),
                rhs: self.rhs.to_string(),
            }),
        });
    }
}

pub struct FanoutBus {
    pub input: BusId,
    pub output: BusId,
}

impl IntermediateRepr for FanoutBus {
    fn to_verilog(&self, module: &mut verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.output.width(),
            assignment: Some(verilog::Expression::Fanout {
                wire: self.input.to_string(),
                output_width: self.output.width(),
            }),
        });
    }
}

pub struct BusShiftLeft {
    pub width: usize,
    pub input: BusId,
    pub output: BusId,
    pub shift: usize,
}

impl IntermediateRepr for BusShiftLeft {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::LeftShift {
                wire: self.input.to_string(),
                amount: self.shift,
            }),
        });
    }
}

pub struct BusShiftRight {
    pub width: usize,
    pub input: BusId,
    pub output: BusId,
    pub shift: usize,
}

impl IntermediateRepr for BusShiftRight {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::RightShift {
                wire: self.input.to_string(),
                amount: self.shift,
            }),
        });
    }
}
