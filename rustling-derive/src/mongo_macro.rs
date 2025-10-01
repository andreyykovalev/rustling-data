extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();
    implement_mongo_repository_trait(&syntax_tree)
}

fn implement_mongo_repository_trait(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident; // Name of the struct
    let (entity, id, collection_name) = get_entity_and_id(syntax_tree);

    let generated = quote! {
        #[async_trait::async_trait]
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
        use mongodb::bson::doc;
        use futures::stream::TryStreamExt; // <- required

        let collection = self.client.database(&self.db_name)
            .collection::<#entity>(#collection_name);

        let mut cursor = collection.find(doc! {}, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

            // async fn find_by_id(&self, id: #id) -> Result<Option<#entity>, anyhow::Error> {
            //     use mongodb::bson::doc;
            //
            //     let collection = self.client.database(&self.db_name)
            //         .collection::<#entity>(#collection_name);
            //     let filter = doc! { "_id": id };
            //     let result = collection.find_one(filter, None).await?;
            //     Ok(result)
            // }
        }
    };
    generated.into()
}

fn get_entity_and_id(
    ast: &syn::DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, String) {
    let entity_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("entity"))
        .expect("Missing #[entity(Type)]");

    let id_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("id"))
        .expect("Missing #[id(Type)]");

    let entity: syn::Type = entity_attr.parse_args().unwrap();
    let id: syn::Type = id_attr.parse_args().unwrap();

    let collection_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("collection"));

    let collection_name = if let Some(attr) = collection_attr {
        attr.parse_args::<syn::LitStr>().unwrap().value()
    } else {
        let ident = match entity {
            syn::Type::Path(ref p) if p.qself.is_none() => {
                p.path.segments.last().unwrap().ident.to_string()
            }
            _ => panic!("Unsupported entity type"),
        };
        ident.to_lowercase() + "s"
    };

    (quote! { #entity }, quote! { #id }, collection_name)
}
