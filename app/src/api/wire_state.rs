use std::ops::Not;

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

impl WireState {
    pub fn and(self, rhs: WireState) -> WireState {
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

    pub fn or(self, rhs: WireState) -> WireState {
        if self == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self, rhs) {
            (WireState::Zero, WireState::Zero) => WireState::Zero,
            (WireState::Zero, WireState::One) => WireState::One,
            (WireState::One, WireState::Zero) => WireState::One,
            (WireState::One, WireState::One) => WireState::One,

            (WireState::One, WireState::Z) => WireState::One,
            (WireState::Z, WireState::One) => WireState::One,

            (WireState::Zero, WireState::Z) => WireState::X,
            (WireState::Z, WireState::Zero) => WireState::X,
            (WireState::Z, WireState::Z) => WireState::X,
            _ => unreachable!("Processed earlier"),
        }
    }

    pub fn xor(self, rhs: WireState) -> WireState {
        if self == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self, rhs) {
            (WireState::Zero, WireState::Zero) => WireState::Zero,
            (WireState::Zero, WireState::One) => WireState::One,
            (WireState::One, WireState::Zero) => WireState::One,
            (WireState::One, WireState::One) => WireState::Zero,

            (WireState::One, WireState::Z) => WireState::X,
            (WireState::Z, WireState::One) => WireState::X,
            (WireState::Zero, WireState::Z) => WireState::X,
            (WireState::Z, WireState::Zero) => WireState::X,
            (WireState::Z, WireState::Z) => WireState::X,
            _ => unreachable!("Processed earlier"),
        }
    }

    pub fn not(self) -> WireState {
        match self {
            WireState::Zero => WireState::One,
            WireState::One => WireState::Zero,
            WireState::Z => WireState::X,
            WireState::X => WireState::X,
        }
    }
}

// TODO: Implement ConstZeroWire and ConstOneWire as buses.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_operators_symmetry() {
        const WIRE_STATE_VARIANTS: [WireState; 4] =
            [WireState::Zero, WireState::One, WireState::X, WireState::Z];

        for i in 0..4 {
            for j in 0..4 {
                let lhs = WIRE_STATE_VARIANTS[i];
                let rhs = WIRE_STATE_VARIANTS[j];

                assert_eq!(lhs.and(rhs), rhs.and(lhs));
                assert_eq!(lhs.or(rhs), rhs.or(lhs));
                assert_eq!(lhs.xor(rhs), rhs.xor(lhs));
            }
        }
    }
}
