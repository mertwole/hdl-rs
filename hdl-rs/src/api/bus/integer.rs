use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

use crate::{api::bus::Bus, intermediate_repr::BusId};

/// Bus representing a W-bit unsigned integer.
trait UnsignedIntegerBus<const W: usize>: Bus<W> {}

/// Bus representing a W-bit signed integer in a two's complement representation.
trait SignedIntegerBus<const W: usize>: Bus<W> {}

trait SignedOps<const W: usize>: SignedIntegerBus<W> {
    fn add<RB: Bus<W>>(self, rhs: RB) -> SignedAddResult<W, Self, RB> {
        SignedAddResult::new(self, rhs)
    }

    fn sub<RB: Bus<W>>(self, rhs: RB) -> SignedSubResult<W, Self, RB> {
        SignedSubResult::new(self, rhs)
    }
}

impl<const W: usize, T: SignedIntegerBus<W>> SignedOps<W> for T {}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W + 1)]
#[derive_clock_bus(BL, BR)]
struct SignedAddResult<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
    bus_id: BusId,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<{ W + 1 }> for SignedAddResult<W, BL, BR> {
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn get_id(self) -> BusId {
        self.bus_id
    }

    fn build_intermediate_repr(
        self,
        builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
        todo!()
    }
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> SignedAddResult<W, BL, BR> {
    fn new(lhs: BL, rhs: BR) -> Self {
        Self {
            lhs,
            rhs,
            bus_id: BusId::new_unique(W + 1),
        }
    }
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W + 1)]
#[derive_clock_bus(BL, BR)]
struct SignedSubResult<const W: usize, BL: Bus<W>, BR: Bus<W>> {
    lhs: BL,
    rhs: BR,
    bus_id: BusId,
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> Bus<{ W + 1 }> for SignedSubResult<W, BL, BR> {
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn get_id(self) -> BusId {
        self.bus_id
    }

    fn build_intermediate_repr(
        self,
        builder: &mut crate::intermediate_repr::IntermediateReprBuilder,
    ) {
        todo!()
    }
}

impl<const W: usize, BL: Bus<W>, BR: Bus<W>> SignedSubResult<W, BL, BR> {
    fn new(lhs: BL, rhs: BR) -> Self {
        Self {
            lhs,
            rhs,
            bus_id: BusId::new_unique(W + 1),
        }
    }
}

// TODO: Unsigned ops.

const fn usize_min(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs { lhs } else { rhs }
}
