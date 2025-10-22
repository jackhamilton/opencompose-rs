use crate::dsl_node::DSLNode;
use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn view_builder_impl(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as DSLNode);

    fn expand(node: &DSLNode) -> proc_macro2::TokenStream {
        let name_str = node.name.to_string();
        let ident = syn::Ident::new(&name_str, proc_macro2::Span::call_site());

        match name_str.as_str() {
            "Row" | "Column" | "Box" => {
                let children = node.children.iter().map(expand);
                quote! {
                    opencompose_rs::ast::OpenComposeAST::Container(Box::new(
                        opencompose_rs::ast::ContainerNode::#ident(
                            opencompose_rs::ast::OpenComposeAST::List(Box::new([
                                #(#children),*
                            ]))
                    )
                    ))
                }
            }

            _ => {
                let arguments = node.arguments.iter().map(|arg| {
                    let _key = &arg.key;
                    let val = &arg.value;
                    quote! { #val }
                });

                let modifier_fields = node.modifiers.iter().map(|m| {
                    let key = &m.key;
                    let val = &m.args.first().expect("Expected one argument");
                    quote! { .#key(#val) }
                });
                let config_ident = format_ident!("{ident}Config");
                quote! {
                    opencompose_rs::ast::OpenComposeAST::View(
                        opencompose_rs::ast::ViewNode::#ident(
                            opencompose_rs::configs::#ident::#config_ident::new(
                                #(#arguments,)*
                            )
                            #(#modifier_fields)*
                        )
                    )
                }
            }
        }
    }

    let expanded = expand(&root);

    TokenStream::from(quote! {
        #expanded
    })
}
