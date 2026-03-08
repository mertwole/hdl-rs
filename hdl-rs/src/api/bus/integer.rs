use crate::api::bus::Bus;

/// Bus representing a W-bit unsigned integer.
pub trait UnsignedIntegerBus<const W: usize>: Bus<W> {}

/// Bus representing a W-bit signed integer in a two's complement representation.
pub trait SignedIntegerBus<const W: usize>: Bus<W> {}
