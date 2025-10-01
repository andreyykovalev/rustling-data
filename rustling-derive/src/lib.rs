extern crate proc_macro;

use proc_macro::TokenStream;

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
