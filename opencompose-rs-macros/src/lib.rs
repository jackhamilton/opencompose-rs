extern crate proc_macro;
use crate::macro_name::macro_name_impl;
use proc_macro::TokenStream;

mod macro_name;

#[proc_macro]
pub fn macro_name(input: TokenStream) -> TokenStream {
    macro_name_impl(input)
}
