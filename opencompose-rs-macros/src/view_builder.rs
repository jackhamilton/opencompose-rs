use syn::token;
use syn::Ident;
use syn::braced;
use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{
    parse::{Parse, ParseStream},
};

struct DSLNode {
    name: Ident,
    children: Vec<DSLNode>,
}

impl Parse for DSLNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut children = Vec::new();

        if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        }

        Ok(DSLNode { name, children })
    }
}

pub fn view_builder_impl(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as DSLNode);

    fn expand(node: &DSLNode) -> proc_macro2::TokenStream {
        let name_str = node.name.to_string();
        let ident = syn::Ident::new(&name_str, proc_macro2::Span::call_site());

        match name_str.as_str() {
            // Handle container nodes
            "Row" | "Column" | "Box" => {
                // Recursively expand children into OpenComposeASTs
                let inner_nodes = node.children.iter().map(expand);
                quote! {
                    crate::ast::OpenComposeAST::Container(Box::new(
                        crate::ast::ContainerNode::#ident(
                            crate::ast::OpenComposeAST::ViewListNode(Box::new([
                                #(#inner_nodes),*
                            ])))
                    ))
                }
            }

            // Handle simple view nodes
            "Image" | "Text" | "Button" => {
                quote! {
                    crate::ast::OpenComposeAST::View(crate::ast::ViewNode::#ident)
                }
            }

            // Unknown node type → compile error
            _ => quote! {
                compile_error!(concat!("Unknown DSL node: ", #name_str))
            },
        }
    }

    let expanded = expand(&root);

    TokenStream::from(quote! {
        #expanded
    })
}

