use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Expr, Field, Ident, Lit, Token, Type, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

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

        let raw_fields = content.parse_terminated(Field::parse_named, Token![,])?;
        let mut fields = Vec::with_capacity(raw_fields.len());
        for field in raw_fields {
            let name = field
                .ident
                .ok_or(syn::Error::new(input.span(), "Missing field name"))?;

            let mut desc = None;
            for attr in &field.attrs {
                if attr.path().is_ident("desc") {
                    desc = parse_desc(attr)?;
                    break;
                }
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

fn parse_desc(attr: &Attribute) -> syn::Result<Option<String>> {
    match &attr.meta {
        syn::Meta::NameValue(nv) => {
            if let Expr::Lit(expr_lit) = &nv.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                    return Ok(Some(lit_str.value()));
                }
            }
            Err(syn::Error::new(nv.value.span(), "Expected string literal"))
        }
        _ => Err(syn::Error::new(
            attr.span(),
            "Expected name-value attribute, e.g. #[desc = \"...\"]",
        )),
    }
}
