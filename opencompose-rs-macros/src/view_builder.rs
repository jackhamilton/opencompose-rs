use crate::dsl_node::DSLNode;
use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn view_builder_impl(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as DSLNode);

    fn expand(node: &DSLNode) -> proc_macro2::TokenStream {
        let name_str = node.name.to_string();
        let ident = syn::Ident::new(&name_str, proc_macro2::Span::call_site());

        let arguments = node.arguments.iter().map(|arg| {
            let _key = &arg.key;
            let val = &arg.value;
            quote! { #val }
        });

        // MARK: All valid view config nodes
        let view_config_idents: Vec<String> = vec!["frame".to_string()];
        // let modifiers: Vec<String> = node.modifiers.iter().map(|a| a.key.to_string()).collect();
        // println!("All modifier keys: {:?}", modifiers);
        let view_modifier_fields = node.modifiers.iter().filter(|m| {
            view_config_idents.contains(&m.key.to_string())
        }).map(|m| {
            let key = &m.key;
            // println!("View modifier registered: {:?}", key);
            let args = m.args.iter().map(|arg| &arg.value);
            quote! { .#key(#(#args,)*) }
        });

        match name_str.as_str() {
            "Row" | "Column" | "Box" | "Button" => {
                let children = node.children.iter().map(expand);

                quote! {
                    opencompose_rs::ast::OpenComposeAST::Container(
                        ViewConfig::new()
                        #(#view_modifier_fields)*
                        .done(),
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
                let modifier_fields = node.modifiers.iter().filter(|m| {
                    !view_config_idents.contains(&m.key.to_string())
                }).map(|m| {
                    let key = &m.key;
                    let args = m.args.iter().map(|arg| &arg.value);
                    // println!("Modifier registered: {:?}", key);
                    quote! { .#key(#(#args,)*) }
                });

                let config_ident = format_ident!("{ident}Config");
                quote! {
                    opencompose_rs::ast::OpenComposeAST::View(
                        ViewConfig::new(),
                        opencompose_rs::ast::ViewNode::#ident(
                            ViewConfig::new()
                            #(#view_modifier_fields)*
                            .done(),
                            opencompose_rs::configs::#ident::#config_ident::new(
                                #(#arguments,)*
                            )
                            #(#modifier_fields)*
                            .done()
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
