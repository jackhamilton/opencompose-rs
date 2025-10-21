extern crate proc_macro;
use crate::view_builder::view_builder_impl;
use proc_macro::TokenStream;

mod view_builder;

#[proc_macro]
pub fn view_builder(input: TokenStream) -> TokenStream {
    view_builder_impl(input)
}
