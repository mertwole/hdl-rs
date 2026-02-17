use itertools::Itertools;
use proc_macro2::TokenStream;
use syn::{
    Attribute, Expr, Fields, FieldsNamed, Generics, Ident, ItemStruct, Type, Visibility,
    parse_quote,
};

// TODO
// accept output widht as an argument to macro
//
// implement constructor creating id automatically
// implement operators
// implement get_id (split Bus trait for that)
// implement build_intermediate_repr (split Bus trait for that)
pub struct TypeInfo {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    fields: FieldsNamed,

    width_expr: Expr,
    inputs: Vec<Input>,
}

struct Input {
    ident: Ident,
    ty: Type,
}

impl TypeInfo {
    // TODO: Error processing.
    pub fn parse(item: ItemStruct, width_expr: Expr) -> Result<Self, ()> {
        let Fields::Named(mut fields) = item.fields else {
            return Err(());
        };

        let mut inputs = vec![];
        fields.named.iter_mut().for_each(|field| {
            let input_attr_id = field
                .attrs
                .iter()
                .find_position(|attr| attr.meta.path().is_ident("input"));
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
            attrs: item.attrs,
            vis: item.vis,
            ident: item.ident,
            generics: item.generics,
            fields,

            width_expr,
            inputs,
        })
    }

    pub fn generate_impls(self) -> TokenStream {
        let struct_vis = self.vis;
        let struct_attrs = self.attrs;
        let struct_ident = self.ident;
        let struct_generics = self.generics.clone();

        let struct_fields = Fields::Named(self.fields);
        let struct_fields = struct_fields.iter();

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut clock_bus_where_clause =
            where_clause.cloned().unwrap_or_else(|| parse_quote!(where));
        self.inputs.iter().for_each(|input| {
            let ty = &input.ty;
            let bound = parse_quote!( #ty : crate::api::bus::ClockBus );
            clock_bus_where_clause.predicates.push(bound);
        });

        let mut reset_bus_where_clause =
            where_clause.cloned().unwrap_or_else(|| parse_quote!(where));
        self.inputs.iter().for_each(|input| {
            let ty = &input.ty;
            let bound = parse_quote!( #ty : crate::api::bus::ResetBus );
            reset_bus_where_clause.predicates.push(bound);
        });

        let width_expr = self.width_expr;

        quote! {
            #(#struct_attrs)*
            #[derive(Copy, Clone)]
            #[autoimpl_operators::derive_bus_bitwise_ops(#width_expr)]
            #struct_vis struct #struct_ident #struct_generics {
                #(#struct_fields),*,
                id: crate::intermediate_repr::BusId
            }

            impl #impl_generics crate::api::bus::ResetBus for #struct_ident #ty_generics #reset_bus_where_clause { }

            impl #impl_generics crate::api::bus::ClockBus for #struct_ident #ty_generics #clock_bus_where_clause { }
        }
    }
}
