use crate::{intermediate_repr::IntermediateRepr, verilog};

use super::BusId;

pub struct And {
    pub width: usize,

    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for And {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::And {
                lhs: self.lhs.to_string(),
                rhs: self.rhs.to_string(),
            }),
        });
    }
}

pub struct Or {
    pub width: usize,

    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Or {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::Or {
                lhs: self.lhs.to_string(),
                rhs: self.rhs.to_string(),
            }),
        });
    }
}

pub struct Xor {
    pub width: usize,

    pub lhs: BusId,
    pub rhs: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Xor {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::Xor {
                lhs: self.lhs.to_string(),
                rhs: self.rhs.to_string(),
            }),
        });
    }
}

pub struct Not {
    pub width: usize,

    pub bus: BusId,

    pub output: BusId,
}

impl IntermediateRepr for Not {
    fn to_verilog(&self, module: &mut crate::verilog::VerilogModule) {
        module.add_wire(verilog::WireDefinition {
            name: self.output.to_string(),
            width: self.width,
            assignment: Some(verilog::Expression::Not {
                wire: self.bus.to_string(),
            }),
        });
    }
}
