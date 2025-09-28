use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, DeriveInput, Ident, Token, Type, parse_macro_input};

// Struct to represent the parsed attribute arguments
struct RepositoryArgs {
    entity: Type,
    id: Type,
}

impl Parse for RepositoryArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entity = None;
        let mut id = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let ty: Type = input.parse()?;

            if key == "entity" {
                entity = Some(ty);
            } else if key == "id" {
                id = Some(ty);
            } else {
                return Err(syn::Error::new_spanned(key, "expected `entity` or `id`"));
            }

            // consume optional comma
            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>();
            }
        }

        Ok(Self {
            entity: entity.ok_or_else(|| input.error("missing `entity`"))?,
            id: id.ok_or_else(|| input.error("missing `id`"))?,
        })
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let repo_name = input.ident;

    // Find the #[repository(...)] attribute
    let attr: &Attribute = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("repository"))
        .expect("A `#[repository(...)]` attribute is required.");

    // Parse arguments into RepositoryArgs
    let args = attr
        .parse_args::<RepositoryArgs>()
        .expect("Invalid #[repository] attribute");

    let entity_ty = args.entity;
    let id_ty = args.id;

    let expanded = quote! {
        impl Repository<#entity_ty, #id_ty> for #repo_name {
            fn find_all(&self) -> Result<Vec<#entity_ty>, DataError> {
                unimplemented!("Repository auto-generated find_all stub")
            }
        }
    };

    expanded.into()
}
