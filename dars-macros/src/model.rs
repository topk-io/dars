use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Field, Ident, LitStr, Token, Type, Visibility, braced,
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
    description: Option<String>,
    vis: Visibility,
    name: Ident,
    fields: Vec<ModelField>,
}

impl Model {
    pub(crate) fn with_args(self, description: impl Into<Option<String>>) -> Self {
        Self {
            description: description.into(),
            ..self
        }
    }
}

impl Parse for Model {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse::<Visibility>()?;
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
        Ok(Model {
            vis,
            name,
            fields,
            description: None,
        })
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = &self.vis;
        let name = &self.name;
        let description = self
            .description
            .as_ref()
            .map(|d| d.as_str().trim())
            .unwrap_or_default();
        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            let ty = &field.ty;
            match &field.desc {
                Some(desc) => {
                    quote! {
                        #[schemars(description = #desc.trim())]
                        pub #name: #ty
                    }
                }
                None => {
                    quote! {
                        pub #name: #ty
                    }
                }
            }
        });
        let fields_names = self.fields.iter().map(|field| {
            let name = LitStr::new(&field.name.to_string(), Span::call_site());
            match &field.desc {
                Some(desc) => {
                    let desc = LitStr::new(desc.trim(), Span::call_site());
                    quote! {
                        dars::Field {
                            name: #name,
                            description: Some(#desc),
                        }
                    }
                }
                None => {
                    quote! {
                        dars::Field {
                            name: #name,
                            description: None,
                        }
                    }
                }
            }
        });
        let expanded = quote! {
            #[derive(Debug, Clone, dars::serde::Serialize, dars::serde::Deserialize, dars::schemars::JsonSchema)]
            #[schemars(description = #description)]
            #vis struct #name {
                #(#fields,)*
            }

            impl dars::Model for #name {
                #[inline]
                fn fields() -> &'static [dars::Field] {
                    &[#(#fields_names,)*]
                }
            }
        };
        tokens.extend(expanded);
    }
}
