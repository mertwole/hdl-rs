use proc_macro2::TokenStream;
use syn::{
    Block, Ident, Lit, Token,
    parse::{Parse, ParseStream},
};

pub struct Attribute {
    _for: Token![for],
    iterator: Ident,
    _in: Token![in],
    from: u64,
    _range: Token![..],
    to: u64,
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

pub fn generate_impl(attr: Attribute, block: Block) -> TokenStream {
    let result: TokenStream = (attr.from..attr.to)
        .map(|i| {
            let const_name = &attr.iterator;

            quote!(
                {
                    const #const_name: u64 = #i;
                    #block
                }
            )
        })
        .collect();

    quote!({ #result })
}
