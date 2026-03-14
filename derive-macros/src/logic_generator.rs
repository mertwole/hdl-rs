use itertools::Itertools;
use proc_macro2::TokenStream;
use syn::{
    FnArg, Ident, ItemFn, Lit, Pat, Token,
    parse::{Parse, ParseStream},
};

pub struct Attribute {
    _for: Token![for],
    iterator: Ident,
    _in: Token![in],
    from: usize,
    _range: Token![..],
    to: usize,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _for = input.parse()?;
        let iterator = input.parse()?;
        let _in = input.parse()?;
        let from: Lit = input.parse()?;
        let _range = input.parse()?;
        let to: Lit = input.parse()?;

        let Lit::Int(from) = from else {
            // TODO: Process error.
            panic!("Expected integer literal as a range start");
        };
        let from = from
            .base10_parse()
            .expect("Failed to parse range start as u64");

        let Lit::Int(to) = to else {
            // TODO: Process error.
            panic!("Expected integer literal as a range end");
        };
        let to = to.base10_parse().expect("Failed to parse range end as u64");

        Ok(Self {
            _for,
            iterator,
            _in,
            from,
            _range,
            to,
        })
    }
}

pub fn generate_impl(attr: Attribute, function: ItemFn) -> TokenStream {
    let old_body = function.block;
    let mut signature = function.sig;

    let mut input_arg = None;

    for (arg_idx, arg) in signature.inputs.iter_mut().enumerate() {
        if let FnArg::Typed(typed) = arg {
            let input = typed
                .attrs
                .iter()
                .find_position(|attr| attr.meta.path().is_ident("input"));

            let Some((input_pos, _)) = input else {
                continue;
            };

            typed.attrs.remove(input_pos);

            let Pat::Ident(arg_ident) = typed.pat.as_ref() else {
                panic!("Ident is expected as a function argument");
            };

            if input_arg.is_some() {
                panic!("Maximum of one #[input] attribute is expected")
            }

            input_arg = Some((arg_ident, arg_idx));
        }
    }

    let fn_name = signature.ident.clone();
    let call_args: Vec<_> = signature
        .inputs
        .clone()
        .into_iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => panic!("Receiver args are not allowed"),
            FnArg::Typed(arg) => arg.pat,
        })
        .collect();

    let fn_body: TokenStream = (attr.from..attr.to)
        .map(|i| {
            quote!(
                // TODO: Rename.
                let input = #fn_name::<#i>(#(#call_args,)*);
            )
        })
        .collect();

    let attrs = function.attrs;
    let vis = function.vis;

    quote!(
        #(#attrs)*
        #vis #signature
        {
            #(#attrs)*
            #vis #signature
            #old_body

            #fn_body
            // TODO: Rename
            input
        }
    )
}
