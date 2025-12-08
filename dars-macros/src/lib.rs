use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Expr, Lit, LitStr, parse_macro_input};

mod signature;

#[proc_macro_attribute]
pub fn Signature(args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as signature::Signature);
    let instruction = parse_macro_input!(args as signature::Instruction);
    let sig = sig.with_instruction(instruction);

    println!("sig: {:?}", sig);

    TokenStream::new()
}
