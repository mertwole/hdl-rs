use super::*;

pub trait Bus<const W: usize>: Clone + Copy {
    fn eval(self) -> [WireState; W];
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}

#[derive(Clone, Copy)]
pub struct Dummy<const W: usize> {}

impl<const W: usize> Bus<W> for Dummy<W> {
    fn eval(self) -> [WireState; W] {
        panic!("Dummy bus was used")
    }
}
