use syn::Expr;
use syn::Token;
use syn::Ident;
use syn::{
    parse::{Parse, ParseStream},
};

pub struct NamedArg {
    pub key: Option<Ident>,
    _colon: Option<Token![:]>,
    pub value: Expr
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _colon: input.parse()?,
            value: input.parse()?,
        })
    }
}
