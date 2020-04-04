extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_error::*;

mod args;

mod bitpattern;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn bitpattern(args: TokenStream, input: TokenStream) -> TokenStream {
    bitpattern::bitpattern(args, input)
}

mod bitfragment;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn bitfragment(args: TokenStream, input: TokenStream) -> TokenStream {
    bitfragment::bitfragment(args, input)
}
