use super::BusId;

pub struct FlipFlop {
    data: BusId,
    reset: BusId,
    set: BusId,
    clock: BusId,

    output: BusId,
}
