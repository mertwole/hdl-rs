// TODO: Test that `BitwiseOps` is derived for all structs implementing `Wire`.
use autoimpl_operators::BitwiseOps;

use crate::api::prelude::*;

pub trait LogicOps: Wire {
    fn and<R: Wire>(&self, rhs: R) -> WireAnd<Self, R> {
        WireAnd { lhs: *self, rhs }
    }

    fn or<R: Wire>(&self, rhs: R) -> WireOr<Self, R> {
        WireOr { lhs: *self, rhs }
    }

    fn xor<R: Wire>(&self, rhs: R) -> WireXor<Self, R> {
        WireXor { lhs: *self, rhs }
    }

    fn not(&self) -> WireNot<Self> {
        WireNot { wire: *self }
    }
}

impl<T: Wire> LogicOps for T {}

#[derive(Clone, Copy, BitwiseOps)]
pub struct WireAnd<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireAnd<L, R> {
    fn eval(&self) -> WireState {
        self.lhs.eval().and(self.rhs.eval())
    }
}

#[derive(Clone, Copy, BitwiseOps)]
pub struct WireOr<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireOr<L, R> {
    fn eval(&self) -> WireState {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        if lhs == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self.lhs.eval(), self.rhs.eval()) {
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
}

#[derive(Clone, Copy, BitwiseOps)]
pub struct WireXor<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireXor<L, R> {
    fn eval(&self) -> WireState {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        if lhs == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self.lhs.eval(), self.rhs.eval()) {
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
}

#[derive(Clone, Copy, BitwiseOps)]
pub struct WireNot<W: Wire> {
    wire: W,
}

impl<W: Wire> Wire for WireNot<W> {
    fn eval(&self) -> WireState {
        match self.wire.eval() {
            WireState::Zero => WireState::One,
            WireState::One => WireState::Zero,
            WireState::Z => WireState::X,
            WireState::X => WireState::X,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wire::mock::*;

    #[test]
    fn test_binary_operators_symmetry() {
        const WIRE_STATE_VARIANTS: [WireState; 4] =
            [WireState::Zero, WireState::One, WireState::X, WireState::Z];
        let mut input_permutations = vec![];
        for i in 0..4 {
            for j in i..4 {
                input_permutations.push((
                    MockWire::new(WIRE_STATE_VARIANTS[i]),
                    MockWire::new(WIRE_STATE_VARIANTS[j]),
                ));
            }
        }

        for &(lhs, rhs) in &input_permutations {
            assert_eq!(
                WireAnd { lhs, rhs }.eval(),
                WireAnd { lhs: rhs, rhs: lhs }.eval()
            );
        }

        for &(lhs, rhs) in &input_permutations {
            assert_eq!(
                WireOr { lhs, rhs }.eval(),
                WireOr { lhs: rhs, rhs: lhs }.eval()
            );
        }

        for &(lhs, rhs) in &input_permutations {
            assert_eq!(
                WireXor { lhs, rhs }.eval(),
                WireXor { lhs: rhs, rhs: lhs }.eval()
            );
        }
    }
}
