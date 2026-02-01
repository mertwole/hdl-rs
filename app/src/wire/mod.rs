use std::ops::Not;

use autoimpl_operators::BitwiseOps;

pub mod bus;
pub mod bus_operators;
pub mod concat;
pub mod flip_flop;
pub mod operators;

pub trait Wire: Clone + Copy {
    fn eval(&self) -> WireState;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WireState {
    One,
    Zero,
    Z,
    X,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogicalWireState {
    Zero,
    One,
}

impl From<LogicalWireState> for WireState {
    fn from(value: LogicalWireState) -> Self {
        match value {
            LogicalWireState::Zero => WireState::Zero,
            LogicalWireState::One => WireState::One,
        }
    }
}

impl Not for LogicalWireState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

// TODO: Move all the ops impl from `operators` here(as well as tests).
impl WireState {
    fn and(self, rhs: WireState) -> WireState {
        if self == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self, rhs) {
            (WireState::Zero, WireState::Zero) => WireState::Zero,
            (WireState::Zero, WireState::One) => WireState::Zero,
            (WireState::One, WireState::Zero) => WireState::Zero,
            (WireState::One, WireState::One) => WireState::One,

            (WireState::Zero, WireState::Z) => WireState::Zero,
            (WireState::Z, WireState::Zero) => WireState::Zero,

            (WireState::One, WireState::Z) => WireState::X,
            (WireState::Z, WireState::One) => WireState::X,
            (WireState::Z, WireState::Z) => WireState::X,
            _ => unreachable!("Processed earlier"),
        }
    }
}

pub trait InputWire: Wire + Clone + Copy {}

#[derive(Clone, Copy, BitwiseOps)]
pub struct ConstZeroWire {}

impl Wire for ConstZeroWire {
    fn eval(&self) -> WireState {
        WireState::Zero
    }
}

impl ConstZeroWire {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, BitwiseOps)]
pub struct ConstOneWire {}

impl Wire for ConstOneWire {
    fn eval(&self) -> WireState {
        WireState::One
    }
}

impl ConstOneWire {
    fn new() -> Self {
        Self {}
    }
}
