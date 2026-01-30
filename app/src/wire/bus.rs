use super::*;

enum Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait Bus<const W: usize>: Clone + Copy {
    fn wire_at_const<const N: usize>() -> impl Wire;

    fn wire_at(index: usize) -> impl Wire;

    fn sub_bus_const<const From: usize, const To: usize>() -> impl Bus<{ To - From }>
    where
        [(); To - From]:,
        [(); W - To]:;

    fn sub_bus<const Width: usize>(from: usize) -> impl Bus<Width>;

    fn append_wire_left(self, wire: impl Wire) -> impl Bus<{ W + 1 }>
    where
        [(); W + 1]:;

    fn append_wire_right(self, wire: impl Wire) -> impl Bus<{ W + 1 }>
    where
        [(); W + 1]:;
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}
