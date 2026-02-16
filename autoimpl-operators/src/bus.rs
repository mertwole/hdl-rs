use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Fields, FieldsNamed, Generics, Ident, ItemStruct, Type, parse_quote, token::Token,
};

// TODO
// mark inputs with #[input]
// accept output widht as an argument to macro
//
// derive clone, copy
// add id field
// implement constructor creating id automatically
// derive ClockBus and ResetBus
// implement operators
// implement get_id (split Bus trait for that)
// implement build_intermediate_repr (split Bus trait for that)
pub struct TypeInfo {
    ident: Ident,
    fields: FieldsNamed,
    generics: Generics,

    inputs: Vec<Input>,
}

struct Input {
    ident: Ident,
    ty: Type,
}

impl TypeInfo {
    // TODO: Error processing.
    pub fn parse(item: ItemStruct) -> Result<Self, ()> {
        let Fields::Named(mut fields) = item.fields else {
            return Err(());
        };

        let mut inputs = vec![];
        fields.named.iter_mut().for_each(|field| {
            let input_attr_id = field
                .attrs
                .iter()
                .find_position(|attr| attr.meta.path().is_ident(""));
            if let Some((id, _)) = input_attr_id {
                field.attrs.remove(id);
                inputs.push(Input {
                    // TODO: Process error.
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                });
            }
        });

        Ok(Self {
            ident: item.ident,
            fields,
            generics: item.generics,

            inputs: vec![],
        })
    }

    pub fn generate_impls(self) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let name = self.ident;

        quote! {
            impl #impl_generics crate::api::bus::ResetBus for #name #ty_generics #where_clause { }
        }
    }
}
