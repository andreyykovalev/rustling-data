extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod common;
mod mongo_macro;
mod postgres_macro;

#[proc_macro_derive(Repository, attributes(entity, id, table))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    postgres_macro::repository_derive(input)
}

#[proc_macro_derive(MongoRepository, attributes(entity, id, collection))]
pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    mongo_macro::mongo_repository_derive(input)
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // Only named structs supported
    let fields: Vec<_> = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .filter(|f| f.ident.as_ref().unwrap() != "id")
                .collect(),
            _ => panic!("Entity derive only supports named fields"),
        },
        _ => panic!("Entity derive only supports structs"),
    };

    let column_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string()).collect();
    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let gene = quote! {
        impl #name {
            pub fn columns() -> &'static [&'static str] {
                &[#(#column_names),*]
            }

            pub fn values<'e>(&'e self) -> Vec<&'e (impl sqlx::Encode<'e, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>)> {
                vec![#(&self.#field_idents),*]
            }
        }
    };

    gene.into()
}