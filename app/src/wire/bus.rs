use super::*;

pub trait Bus<const W: usize>: Clone + Copy {
    fn eval(self) -> [WireState; W];
}

pub trait InputBus<const W: usize>: Bus<W> + Clone + Copy {}
