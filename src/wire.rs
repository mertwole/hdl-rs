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

pub trait InputWire: Wire + Clone + Copy {}

#[derive(Clone, Copy)]
pub struct WireAnd<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireAnd<L, R> {
    fn eval(&self) -> WireState {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        if lhs == WireState::X || rhs == WireState::X {
            return WireState::X;
        }

        match (self.lhs.eval(), self.rhs.eval()) {
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

pub trait LogicOps: Wire {
    fn and<R: Wire>(&self, rhs: R) -> WireAnd<Self, R> {
        WireAnd { lhs: *self, rhs }
    }

    fn or<R: Wire>(&self, rhs: R) -> WireOr<Self, R> {
        WireOr { lhs: *self, rhs }
    }

    fn not(&self) -> WireNot<Self> {
        WireNot { wire: *self }
    }
}

impl<T: Wire> LogicOps for T {}
