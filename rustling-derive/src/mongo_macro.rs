use crate::common::parse_repository_meta;
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();
    implement_mongo_repository_trait(&syntax_tree)
}

fn implement_mongo_repository_trait(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident;
    let meta = parse_repository_meta(syntax_tree, "collection");

    let entity = meta.entity;
    let id = meta.id;
    let storage_name = syn::LitStr::new(&meta.storage_name, proc_macro2::Span::call_site());

    let generated = quote! {
        #[async_trait::async_trait]
        impl ::rustling_data::api::CrudRepository<#entity, #id, ::rustling_data::api::MongoError> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, ::rustling_data::api::RepositoryError<::rustling_data::api::MongoError>> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                mongo_repo.find_all::<#entity>(#storage_name).await
            }

            async fn find_one(&self, id: &#id) -> Result<Option<#entity>, ::rustling_data::api::RepositoryError<::rustling_data::api::MongoError>> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                let filter = ::rustling_data::bson::doc! { "_id": id };
                mongo_repo.find_one::<#entity>(#storage_name, filter).await
            }

            async fn insert_one(&self, doc: &#entity) -> Result<::rustling_data::bson::oid::ObjectId, ::rustling_data::api::RepositoryError<::rustling_data::api::MongoError>> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                mongo_repo.insert_one(#storage_name, doc).await
            }

            async fn update_one(&self, id: &#id, doc: &#entity) -> Result<Option<#entity>, ::rustling_data::api::RepositoryError<::rustling_data::api::MongoError>> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                let filter = ::rustling_data::bson::doc! { "_id": id };
                mongo_repo.update_one(#storage_name, filter, doc).await
            }

            async fn delete_one(&self, id: &#id) -> Result<u64, ::rustling_data::api::RepositoryError<::rustling_data::api::MongoError>> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                let filter = ::rustling_data::bson::doc! { "_id": id };
                mongo_repo.delete_one(#storage_name, filter).await
            }
        }
    };

    generated.into()
}
