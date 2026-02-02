#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

// TODO: Add tests.

use syn::{DeriveInput, parse_macro_input};

mod bus_bitwise_ops;
mod wire_bitwise_ops;

#[proc_macro_derive(WireBitwiseOps)]
pub fn derive_wire_bitwise_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let type_info = parse_macro_input!(input as DeriveInput);
    let type_info = wire_bitwise_ops::TypeInfo::from_derive_input(type_info);

    let operators_impl = type_info.generate_operators_impl();

    proc_macro::TokenStream::from(operators_impl)
}

// TODO: Turn into attribute macro.
#[proc_macro_derive(BusBitwiseOps)]
pub fn derive_bus_bitwise_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let type_info = parse_macro_input!(input as DeriveInput);
    let type_info = bus_bitwise_ops::TypeInfo::from_derive_input(type_info);

    let operators_impl = type_info.generate_operators_impl();

    proc_macro::TokenStream::from(operators_impl)
}
