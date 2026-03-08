use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

use crate::{
    api::bus::Bus,
    intermediate_repr::{BusId, IntermediateReprBuilder},
};

/// Bus representing a W-bit unsigned integer.
pub trait UnsignedIntegerBus<const W: usize>: Bus<W> {}

/// Bus representing a W-bit signed integer in a two's complement representation.
pub trait SignedIntegerBus<const W: usize>: Bus<W> {}

pub fn signed_integer_adder<const W: usize, LB: SignedIntegerBus<W>, RB: SignedIntegerBus<W>>(
    lhs: LB,
    rhs: RB,
) -> SignedAddResult<W, LB, RB> {
    SignedAddResult::new(lhs, rhs)
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W + 1)]
#[derive_clock_bus(BL, BR)]
pub struct SignedAddResult<const W: usize, BL: SignedIntegerBus<W>, BR: SignedIntegerBus<W>> {
    lhs: BL,
    rhs: BR,
    bus_id: BusId,
}

impl<const W: usize, BL: SignedIntegerBus<W>, BR: SignedIntegerBus<W>> SignedIntegerBus<{ W + 1 }>
    for SignedAddResult<W, BL, BR>
{
}

impl<const W: usize, BL: SignedIntegerBus<W>, BR: SignedIntegerBus<W>> Bus<{ W + 1 }>
    for SignedAddResult<W, BL, BR>
{
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

impl<const W: usize, BL: SignedIntegerBus<W>, BR: SignedIntegerBus<W>> SignedAddResult<W, BL, BR> {
    fn new(lhs: BL, rhs: BR) -> Self {
        Self {
            lhs,
            rhs,
            bus_id: BusId::new_unique(W + 1),
        }
    }
}

pub fn unsigned_integer_adder<
    const W: usize,
    LB: UnsignedIntegerBus<W>,
    RB: UnsignedIntegerBus<W>,
>(
    lhs: LB,
    rhs: RB,
) -> UnsignedAddResult<W, LB, RB> {
    UnsignedAddResult::new(lhs, rhs)
}

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W + 1)]
#[derive_clock_bus(BL, BR)]
pub struct UnsignedAddResult<const W: usize, BL: UnsignedIntegerBus<W>, BR: UnsignedIntegerBus<W>> {
    lhs: BL,
    rhs: BR,
    bus_id: BusId,
}

impl<const W: usize, BL: UnsignedIntegerBus<W>, BR: UnsignedIntegerBus<W>>
    UnsignedIntegerBus<{ W + 1 }> for UnsignedAddResult<W, BL, BR>
{
}

impl<const W: usize, BL: UnsignedIntegerBus<W>, BR: UnsignedIntegerBus<W>> Bus<{ W + 1 }>
    for UnsignedAddResult<W, BL, BR>
{
    const COMBINATIONAL_NETWORK_ID: usize =
        usize_min(BL::COMBINATIONAL_NETWORK_ID, BR::COMBINATIONAL_NETWORK_ID);

    fn get_id(self) -> BusId {
        self.bus_id
    }

    fn build_intermediate_repr(self, builder: &mut IntermediateReprBuilder) {
        todo!()
    }
}

impl<const W: usize, BL: UnsignedIntegerBus<W>, BR: UnsignedIntegerBus<W>>
    UnsignedAddResult<W, BL, BR>
{
    fn new(lhs: BL, rhs: BR) -> Self {
        Self {
            lhs,
            rhs,
            bus_id: BusId::new_unique(W + 1),
        }
    }
}

const fn usize_min(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs { lhs } else { rhs }
}
