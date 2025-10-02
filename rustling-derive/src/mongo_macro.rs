use proc_macro::TokenStream;
use quote::quote;
use syn;
use crate::common::parse_repository_meta;

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
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
                use mongodb::bson::doc;
                use futures::stream::TryStreamExt;

                let collection = self.client.database(&self.db_name)
                    .collection::<#entity>(#storage_name);

                let mut cursor = collection.find(doc! {}, None).await?;
                let mut results = Vec::new();

                while let Some(doc) = cursor.try_next().await? {
                    results.push(doc);
                }

                Ok(results)
            }
        }
    };
    generated.into()
}
