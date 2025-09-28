// rustling-data-derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Ident, Type, Token};
use syn::parse::{Parse, ParseStream};

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

#[proc_macro_derive(Repository, attributes(repository))]
pub fn derive_repository(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attr: &Attribute = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("repository"))
        .expect("A `#[repository(...)]` attribute is required.");

    let args = attr
        .parse_args::<RepositoryArgs>()
        .expect("Invalid #[repository] attribute");

    let entity_ty = args.entity;
    let id_ty = args.id;
    let repo_name = input.ident;

    let expanded = quote! {
        impl rustling_api::Repository<#entity_ty, #id_ty> for SqlRepository<'_, #entity_ty> {
            fn find_all(&self) -> Result<Vec<#entity_ty>, rustling_api::DataError> {
                unimplemented!("Repository auto-generated find_all stub")
            }
        }
    };

    expanded.into()
}
