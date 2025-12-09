use quote::ToTokens;
use syn::{Attribute, Expr, Lit, MetaNameValue, spanned::Spanned};

pub(crate) fn parse_desc(attr: &Attribute) -> syn::Result<Option<String>> {
    println!("attr: {:?}", attr.to_token_stream());
    let args = attr.parse_args::<MetaNameValue>()?;
    match args.path.get_ident() {
        Some(ident) => {
            if ident == "desc" {
                match args.value {
                    Expr::Lit(lit) => match lit.lit {
                        Lit::Str(str) => Ok(Some(str.value())),
                        _ => return Err(syn::Error::new(lit.span(), "Expected string literal")),
                    },
                    _ => {
                        return Err(syn::Error::new(
                            args.value.span(),
                            "Expected string literal",
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Invalid parameter: {ident}"),
                ));
            }
        }
        None => {
            return Err(syn::Error::new(attr.span(), "Missing attribute name"));
        }
    }
}
