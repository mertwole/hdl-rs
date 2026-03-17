use proc_macro2::{Punct, TokenStream, TokenTree};
use syn::{
    Block, Expr, FnArg, GenericParam, Ident, ItemFn, Lit, Pat, Token,
    parse::{Parse, ParseStream},
    parse_quote,
};

pub struct Item {
    from: usize,
    to: usize,
    initial_assignments: Block,
    body: Block,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from: Lit = input.parse()?;
        let _range: Token![..] = input.parse()?;
        let to: Lit = input.parse()?;
        let initial_assignments = input.parse()?;
        let body = input.parse()?;

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
            from,
            to,
            initial_assignments,
            body,
        })
    }
}

pub fn generate_impl(item: Item) -> TokenStream {
    let body = item.body.stmts;

    let mut loop_logic = quote!();
    for i in item.from..item.to {
        loop_logic = quote! {
            macro_rules! iterator_literal (
                () => {
                    #i
                }
            );

            #(#body)*
            #loop_logic
        };
    }

    let initial_assignments = item.initial_assignments.stmts;

    quote! {
        #(#initial_assignments)*

        #loop_logic
    }
}
