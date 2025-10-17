use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::parse_repository_meta;

pub fn repository_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    implement_repository_trait(&ast)
}

fn implement_repository_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let meta = parse_repository_meta(ast, "table");

    let entity_type = meta.entity;
    let id_type = meta.id;
    let table_name = syn::LitStr::new(&meta.storage_name, proc_macro2::Span::call_site());

    let gene = quote! {
        #[async_trait::async_trait]
        impl rustling_data::api::CrudRepository<#entity_type, #id_type, sqlx::Error> for #name {
            async fn find_all(&self) -> Result<Vec<#entity_type>, rustling_data::api::RepositoryError<sqlx::Error>> {
                rustling_data::PostgresDriver::find_all(&self.pool, #table_name).await
            }

            async fn find_one(&self, id: &#id_type) -> Result<Option<#entity_type>, rustling_data::api::RepositoryError<sqlx::Error>> {
                rustling_data::PostgresDriver::find_one(&self.pool, #table_name, "id", *id).await
            }

            async fn insert_one(&self, entity: &#entity_type) -> Result<#id_type, rustling_data::api::RepositoryError<sqlx::Error>> {
                let columns = #entity_type::columns();
                let values = entity.values();
                rustling_data::PostgresDriver::insert(&self.pool, #table_name, columns, values).await
            }

            async fn update_one(&self, id: &#id_type, entity: &#entity_type) -> Result<Option<#entity_type>, rustling_data::api::RepositoryError<sqlx::Error>> {
                let columns = #entity_type::columns();
                let values = entity.values();
                let affected = rustling_data::PostgresDriver::update(&self.pool, #table_name, "id", *id, columns, values).await?;
                if affected > 0 {
                    self.find_one(id).await
                } else {
                    Ok(None)
                }
            }

            async fn delete_one(&self, id: &#id_type) -> Result<u64, rustling_data::api::RepositoryError<sqlx::Error>> {
                rustling_data::PostgresDriver::delete(&self.pool, #table_name, "id", *id).await
            }
        }
    };

    gene.into()
}

