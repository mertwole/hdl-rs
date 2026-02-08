use super::BusId;

pub struct And {
    lhs: BusId,
    rhs: BusId,

    output: BusId,
}

pub struct Or {
    lhs: BusId,
    rhs: BusId,

    output: BusId,
}

pub struct Xor {
    lhs: BusId,
    rhs: BusId,

    output: BusId,
}

pub struct Not {
    bus: BusId,

    output: BusId,
}
