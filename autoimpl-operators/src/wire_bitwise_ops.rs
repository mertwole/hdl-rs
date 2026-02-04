use proc_macro2::TokenStream;
use syn::{DeriveInput, Generics, Ident, parse_quote};

pub struct TypeInfo {
    name: Ident,
    generics: Generics,
}

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
            .push(parse_quote!(_T: Wire));

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::ops::Not for #name #ty_generics #where_clause {
                type Output = crate::api::wire::WireNot<#name #ty_generics>;

                fn not(self) -> Self::Output {
                    use crate::api::wire::LogicOps;

                    self.not()
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitAnd<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::wire::WireAnd<#name #ty_generics, _T>;

                fn bitand(self, rhs: _T) -> Self::Output {
                    use crate::api::wire::LogicOps;

                    self.and(rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitOr<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::wire::WireOr<#name #ty_generics, _T>;

                fn bitor(self, rhs: _T) -> Self::Output {
                    use crate::api::wire::LogicOps;

                    self.or(rhs)
                }
            }

            impl #impl_generics_with_added_t ::std::ops::BitXor<_T> for #name #ty_generics #where_clause {
                type Output = crate::api::wire::WireXor<#name #ty_generics, _T>;

                fn bitxor(self, rhs: _T) -> Self::Output {
                    use crate::api::wire::LogicOps;

                    self.xor(rhs)
                }
            }
        }
    }
}
