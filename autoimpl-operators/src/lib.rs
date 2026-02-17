#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

// TODO: Add tests.

use syn::{Expr, ItemStruct, Token, Type, parse_macro_input, punctuated::Punctuated};

mod bus;
mod bus_bitwise_ops;

#[proc_macro_attribute]
pub fn derive_bus_bitwise_ops(
    attr: proc_macro::TokenStream,
    mut item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = item.clone();
    let item_struct = parse_macro_input!(input as ItemStruct);

    let width_expr = parse_macro_input!(attr as Expr);

    let type_info = bus_bitwise_ops::TypeInfo::new(item_struct, width_expr);

    let operators_impl = type_info.generate_operators_impl();

    item.extend(proc_macro::TokenStream::from(operators_impl));
    item
}

#[proc_macro_attribute]
pub fn bus(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    // TODO: Process errors.
    let impls = bus::TypeInfo::parse(item_struct).unwrap().generate_impls();
    proc_macro::TokenStream::from(impls)
}
