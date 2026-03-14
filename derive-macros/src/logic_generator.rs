use itertools::Itertools;
use proc_macro2::TokenStream;
use syn::{
    FnArg, GenericParam, Ident, ItemFn, Lit, Pat, Token,
    parse::{Parse, ParseStream},
    parse_quote,
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
    let mut signature = function.sig;

    // Determine input argument position(if any).
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

            input_arg = Some(arg_idx);
        }
    }

    let inner_signature = signature.clone();

    // Generate call args for the inner fn.
    let fn_name = inner_signature.ident.clone();
    let call_args: Vec<_> = inner_signature
        .inputs
        .clone()
        .into_iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => panic!("Receiver args are not allowed"),
            FnArg::Typed(arg) => arg.pat.as_ref().clone(),
        })
        .collect();

    // Call the inner fn multiple times.
    let input_arg_name = input_arg
        .map(|idx| call_args[idx].clone())
        .unwrap_or(parse_quote!(_));
    let fn_body: TokenStream = (attr.from..attr.to)
        .map(|i| {
            quote!(
                let #input_arg_name = #fn_name::<#i>(#(#call_args,)*);
            )
        })
        .collect();

    // Remove the iterator generic from the outer function signature.
    signature.generics.params = signature
        .generics
        .params
        .into_iter()
        .filter(|param| {
            let GenericParam::Const(const_param) = param else {
                return true;
            };

            const_param.ident != attr.iterator
        })
        .collect();
    // TODO: Allow generics where iterator const is not present.
    signature.generics.where_clause = None;

    let attrs = function.attrs;
    let vis = function.vis;
    let inner_body = function.block;

    quote!(
        #(#attrs)*
        #vis #signature
        {
            #(#attrs)*
            #vis #inner_signature
            #inner_body

            #fn_body

            #input_arg_name
        }
    )
}
