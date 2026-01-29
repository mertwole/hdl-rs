#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Generics, Ident, parse_macro_input, parse_quote};

#[proc_macro_derive(BitwiseOps)]
pub fn implement_cache(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let type_info = parse_macro_input!(input as DeriveInput);
    let type_info = TypeInfo::from_derive_input(type_info);

    let operators_impl = type_info.generate_operators_impl();

    proc_macro::TokenStream::from(operators_impl)
}

struct TypeInfo {
    name: Ident,
    generics: Generics,
}

impl TypeInfo {
    fn from_derive_input(item: DeriveInput) -> Self {
        Self {
            name: item.ident,
            generics: item.generics,
        }
    }

    fn generate_operators_impl(self) -> TokenStream {
        let name = self.name;

        let mut impl_generics = self.generics.clone();
        impl_generics.params.push(parse_quote!(_T: Wire));

        let (_, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::ops::BitAnd<_T> for #name #ty_generics #where_clause {
                type Output = crate::wire::operators::WireAnd<#name #ty_generics, _T>;

                fn bitand(self, rhs: _T) -> Self::Output {
                    use crate::wire::operators::LogicOps;

                    self.and(rhs)
                }
            }

            impl #impl_generics ::std::ops::BitOr<_T> for #name #ty_generics #where_clause {
                type Output = crate::wire::operators::WireOr<#name #ty_generics, _T>;

                fn bitor(self, rhs: _T) -> Self::Output {
                    use crate::wire::operators::LogicOps;

                    self.or(rhs)
                }
            }

            impl #impl_generics ::std::ops::BitXor<_T> for #name #ty_generics #where_clause {
                type Output = crate::wire::operators::WireXor<#name #ty_generics, _T>;

                fn bitxor(self, rhs: _T) -> Self::Output {
                    use crate::wire::operators::LogicOps;

                    self.xor(rhs)
                }
            }
        }
    }
}
