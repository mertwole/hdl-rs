#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

// TODO: Add tests.

use syn::{Block, Expr, ItemStruct, Token, Type, parse_macro_input, punctuated::Punctuated};

mod bus_bitwise_ops;
mod clock_bus;
mod logic_generator;

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
pub fn derive_clock_bus(
    attr: proc_macro::TokenStream,
    mut item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = item.clone();
    let item_struct = parse_macro_input!(input as ItemStruct);

    let child_buses: Vec<Type> =
        parse_macro_input!(attr with Punctuated::<Type, Token![,]>::parse_terminated)
            .into_iter()
            .collect();
    let type_info = clock_bus::TypeInfo::new(item_struct, child_buses);

    let trait_impl = type_info.generate_trait_impl();

    item.extend(proc_macro::TokenStream::from(trait_impl));
    item
}

#[proc_macro_attribute]
pub fn logic_generator(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = item.clone();

    let function = parse_macro_input!(input as syn::ItemFn);
    let attribute = parse_macro_input!(attr as logic_generator::Attribute);

    proc_macro::TokenStream::from(logic_generator::generate_impl(attribute, function))
}
