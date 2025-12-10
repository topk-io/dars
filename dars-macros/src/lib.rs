use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod args;
mod model;
mod signature;
mod util;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Signature(args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as signature::Signature);
    let args = parse_macro_input!(args as args::Args);
    let sig = sig.with_instruction(args);
    TokenStream::from(quote!(#sig))
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Model(args: TokenStream, input: TokenStream) -> TokenStream {
    let model = parse_macro_input!(input as model::Model);
    let args = parse_macro_input!(args as args::Args);
    let model = model.with_args(args);
    TokenStream::from(quote!(#model))
}
