use syn::punctuated::Punctuated;
use syn::parenthesized;
use syn::Token;
use crate::modifier::Modifier;
use crate::named_arg::NamedArg;
use syn::token;
use syn::Ident;
use syn::braced;
use syn::{
    parse::{Parse, ParseStream},
};

pub struct DSLNode {
    pub name: Ident,
    pub arguments: Vec<NamedArg>,
    pub modifiers: Vec<Modifier>,
    pub children: Vec<DSLNode>,
}

impl Parse for DSLNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut arguments = Vec::new();
        let mut modifiers = Vec::new();
        let mut children = Vec::new();
        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            arguments = Punctuated::<NamedArg, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();
        }

        while input.peek(Token![.]) {
            modifiers.push(input.parse()?);
        }
        if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        }

        Ok(DSLNode {
            name,
            arguments,
            modifiers,
            children
        })
    }
}
