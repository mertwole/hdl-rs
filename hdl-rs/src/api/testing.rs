use derive_macros::{derive_bus_bitwise_ops, derive_clock_bus};

use crate::{
    api::prelude::*,
    intermediate_repr::{self, BusId},
};

#[derive(Clone, Copy)]
#[derive_bus_bitwise_ops(W)]
#[derive_clock_bus]
pub struct TestInputBus<const W: usize> {
    id: BusId,
}

impl<const W: usize> TestInputBus<W> {
    pub fn new() -> Self {
        Self {
            id: BusId::new_unique(W),
        }
    }
}

impl<const W: usize> Default for TestInputBus<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize> InputBus<W> for TestInputBus<W> {}

impl<const W: usize> Bus<W> for TestInputBus<W> {
    const COMBINATIONAL_NETWORK_ID: usize = 0;

    fn get_id(self) -> BusId {
        self.id
    }

    fn build_intermediate_repr(self, builder: &mut intermediate_repr::IntermediateReprBuilder) {
        builder.push_element(intermediate_repr::Gate::Input(
            intermediate_repr::InputBus { id: self.id },
        ));
    }
}

macro_rules! impl_int_input_bus {
    ($name:ident, $width:expr, $trait:ty) => {
        #[derive(Clone, Copy)]
        #[derive_bus_bitwise_ops($width)]
        #[derive_clock_bus]
        pub struct $name {
            id: BusId,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    id: BusId::new_unique($width),
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl InputBus<$width> for $name {}

        impl $trait for $name {}

        impl Bus<$width> for $name {
            const COMBINATIONAL_NETWORK_ID: usize = 0;

            fn get_id(self) -> BusId {
                self.id
            }

            fn build_intermediate_repr(
                self,
                builder: &mut intermediate_repr::IntermediateReprBuilder,
            ) {
                builder.push_element(intermediate_repr::Gate::Input(
                    intermediate_repr::InputBus { id: self.id },
                ));
            }
        }
    };
}

impl_int_input_bus!(TestInputBusU8, 8, UnsignedIntegerBus<8>);
impl_int_input_bus!(TestInputBusU16, 16, UnsignedIntegerBus<16>);
impl_int_input_bus!(TestInputBusU132, 32, UnsignedIntegerBus<32>);
impl_int_input_bus!(TestInputBusU64, 64, UnsignedIntegerBus<64>);
impl_int_input_bus!(TestInputBusI8, 8, SignedIntegerBus<8>);
impl_int_input_bus!(TestInputBusI16, 16, SignedIntegerBus<16>);
impl_int_input_bus!(TestInputBusI32, 32, SignedIntegerBus<32>);
impl_int_input_bus!(TestInputBusI64, 64, SignedIntegerBus<64>);
