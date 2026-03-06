use proc_macro2::TokenStream;
use syn::{Generics, Ident, ItemStruct, Type, parse_quote};

pub struct TypeInfo {
    name: Ident,
    generics: Generics,
    child_buses: Vec<Type>,
}

impl TypeInfo {
    pub fn new(item: ItemStruct, child_buses: Vec<Type>) -> Self {
        Self {
            name: item.ident,
            generics: item.generics,
            child_buses,
        }
    }

    pub fn generate_trait_impl(self) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote!(where));

        let additional_bounds = self
            .child_buses
            .iter()
            .map(|bus| parse_quote!( #bus : crate::api::bus::ClockBus ));
        for bound in additional_bounds {
            where_clause.predicates.push(bound);
        }

        let name = self.name;

        quote! {
            impl #impl_generics crate::api::bus::ClockBus for #name #ty_generics #where_clause { }
        }
    }
}
