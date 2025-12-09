use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Field, Ident, Token, Type, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::util::parse_desc;

struct ModelField {
    name: Ident,
    ty: Type,
    desc: Option<String>,
}

pub struct Model {
    name: Ident,
    fields: Vec<ModelField>,
}

impl Parse for Model {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;

        // Extract the content of the struct
        let content;
        braced!(content in input);

        // Parse fields
        let raw_fields = content.parse_terminated(Field::parse_named, Token![,])?;
        let mut fields = Vec::with_capacity(raw_fields.len());
        for field in raw_fields {
            let name = field
                .ident
                .ok_or(syn::Error::new(input.span(), "Missing field name"))?;

            let mut desc = None;
            for attr in &field.attrs {
                if attr.path().is_ident("field") {
                    desc = parse_desc(attr)?;
                    break;
                }
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Unknown attribute on field {name}"),
                ));
            }

            fields.push(ModelField {
                name,
                ty: field.ty,
                desc,
            });
        }
        Ok(Model { name, fields })
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        // println!("model name: {:?}", name);
        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            let ty = &field.ty;
            quote! {
                pub #name: #ty,
            }
        });
        let expanded = quote! {
            #[derive(Debug, dars::serde::Serialize, dars::serde::Deserialize, dars::schemars::JsonSchema)]
            struct #name {
                #(#fields)*
            }
        };
        tokens.extend(expanded);
    }
}
