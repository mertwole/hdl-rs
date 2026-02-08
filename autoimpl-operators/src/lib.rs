#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

// TODO: Add tests.

use syn::{Expr, ItemStruct, parse_macro_input};

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
