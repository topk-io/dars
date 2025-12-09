use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod model;
mod signature;
mod util;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Signature(args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as signature::Signature);
    let instruction = parse_macro_input!(args as signature::Instruction);
    let sig = sig.with_instruction(instruction);
    TokenStream::from(quote!(#sig))
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Model(_args: TokenStream, input: TokenStream) -> TokenStream {
    let model = parse_macro_input!(input as model::Model);
    TokenStream::from(quote!(#model))
}
