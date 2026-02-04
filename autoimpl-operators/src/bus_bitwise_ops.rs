use proc_macro2::TokenStream;
use syn::{Expr, Generics, Ident, ItemStruct, parse_quote};

pub struct TypeInfo {
    name: Ident,
    generics: Generics,
    width_expr: Expr,
}

impl TypeInfo {
    pub fn new(item: ItemStruct, width_expr: Expr) -> Self {
        Self {
            name: item.ident,
            generics: item.generics,
            width_expr,
        }
    }

    pub fn generate_operators_impl(self) -> TokenStream {
        let name = self.name;
        let width_expr = self.width_expr;

        let mut impl_generics_with_added_t = self.generics.clone();
        impl_generics_with_added_t
            .params
            .push(parse_quote!(_T: Bus<{ #width_expr }>));

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let where_clause = match where_clause.cloned() {
            Some(mut clause) => {
                clause.predicates.push(parse_quote!([(); { #width_expr }]:));
                clause
            }
            None => {
                parse_quote!(where [(); { #width_expr }]:)
            }
        };

        quote! {
            impl #impl_generics ::std::ops::Not for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusNot<{#width_expr}, #name #ty_generics>;

                fn not(self) -> Self::Output {
                    crate::api::bus::BusOps::not(self)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitAnd<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusAnd<{#width_expr}, #name #ty_generics, _T>;

                fn bitand(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::and(self, rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitOr<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusOr<{#width_expr}, #name #ty_generics, _T>;

                fn bitor(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::or(self, rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitXor<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::bus::BusXor<{#width_expr}, #name #ty_generics, _T>;

                fn bitxor(self, rhs: _T) -> Self::Output {
                    crate::api::bus::BusOps::xor(self, rhs)
                }
            }
        }
    }
}
