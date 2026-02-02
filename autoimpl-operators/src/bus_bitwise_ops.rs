use proc_macro2::TokenStream;
use syn::{DeriveInput, Generics, Ident, parse_quote};

pub struct TypeInfo {
    name: Ident,
    generics: Generics,
}

// TODO: Get rid of assumption that the first generic to the operator structs will be named W.
impl TypeInfo {
    pub fn from_derive_input(item: DeriveInput) -> Self {
        Self {
            name: item.ident,
            generics: item.generics,
        }
    }

    pub fn generate_operators_impl(self) -> TokenStream {
        let name = self.name;

        let mut impl_generics_with_added_t = self.generics.clone();
        impl_generics_with_added_t
            .params
            .push(parse_quote!(_T: Bus<W>));

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::ops::Not for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusNot<W, #name #ty_generics>;

                fn not(self) -> Self::Output {
                    crate::api::bus::BusOps::not(self)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitAnd<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusAnd<W, #name #ty_generics, _T>;

                fn bitand(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::and(self, rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitOr<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusOr<W, #name #ty_generics, _T>;

                fn bitor(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::or(self, rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitXor<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusXor<W, #name #ty_generics, _T>;

                fn bitxor(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::xor(self, rhs)
                }
            }
        }
    }
}
