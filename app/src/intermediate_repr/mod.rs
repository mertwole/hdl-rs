pub mod flip_flop;
pub mod gates;

pub struct BusId {
    id: usize,
}

pub struct Module {
    inputs: Vec<BusId>,
    outputs: Vec<BusId>,
}
