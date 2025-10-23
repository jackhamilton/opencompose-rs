use syn::parenthesized;
use syn::punctuated::Punctuated;
use syn::token;
use syn::Token;
use syn::Ident;
use syn::{
    parse::{Parse, ParseStream},
};

use crate::named_arg::NamedArg;

pub struct Modifier {
    _dot: Token![.],
    pub key: Ident,
    _paren: token::Paren,
    pub args: Punctuated<NamedArg, Token![,]>,
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _dot: Token![.] = input.parse()?;
        let key: Ident = input.parse()?;
        let content;
        let _paren = parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;
        Ok(Self { _dot, key, _paren, args })
    }
}
