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

pub trait InputWire: Wire + Clone + Copy {}
