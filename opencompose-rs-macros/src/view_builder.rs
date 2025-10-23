use crate::dsl_node::DSLNode;
use crate::named_arg::NamedArg;
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
                    opencompose_rs::ast::OpenComposeAST::Container(
                        ViewConfig::new(),
                        Box::new(
                            opencompose_rs::ast::ContainerNode::#ident(
                                ViewConfig::new(),
                                opencompose_rs::ast::OpenComposeAST::List(
                                    ViewConfig::new(),
                                    Box::new([
                                        #(#children),*
                                    ])
                                )
                            )
                        )
                    )
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
                    let args = m.args.iter().map(|arg| &arg.value);
                    // TODO: Doesn't handle multiple arguments
                    quote! { .#key(#(#args,)*) }
                });
                let config_ident = format_ident!("{ident}Config");
                quote! {
                    opencompose_rs::ast::OpenComposeAST::View(
                        ViewConfig::new(),
                        opencompose_rs::ast::ViewNode::#ident(
                            ViewConfig::new(),
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
