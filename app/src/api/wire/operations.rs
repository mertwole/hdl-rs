// TODO: Test that `WireBitwiseOps` is derived for all structs implementing `Wire`.
use autoimpl_operators::WireBitwiseOps;

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

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct WireAnd<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireAnd<L, R> {
    fn eval(&self) -> WireState {
        self.lhs.eval().and(self.rhs.eval())
    }
}

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct WireOr<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireOr<L, R> {
    fn eval(&self) -> WireState {
        self.lhs.eval().or(self.rhs.eval())
    }
}

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct WireXor<L: Wire, R: Wire> {
    lhs: L,
    rhs: R,
}

impl<L: Wire, R: Wire> Wire for WireXor<L, R> {
    fn eval(&self) -> WireState {
        self.lhs.eval().xor(self.rhs.eval())
    }
}

#[derive(Clone, Copy, WireBitwiseOps)]
pub struct WireNot<W: Wire> {
    wire: W,
}

impl<W: Wire> Wire for WireNot<W> {
    fn eval(&self) -> WireState {
        self.wire.eval().not()
    }
}
