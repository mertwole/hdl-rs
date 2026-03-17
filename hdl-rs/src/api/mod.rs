pub mod bus;
pub mod concat;
pub mod flip_flop;
pub mod wire_state;

#[cfg(feature = "testing")]
pub mod testing;

pub use derive_macros::logic_generator;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::bus::*;
    pub use super::concat::*;
    pub use super::flip_flop::*;
    pub use super::wire_state::*;
    pub use crate::concat_buses;

    pub use derive_macros::const_for_loop;
    pub use derive_macros::logic_generator;
}
