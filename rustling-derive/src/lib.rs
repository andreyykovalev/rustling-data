extern crate proc_macro;

use proc_macro::TokenStream;

mod postgres_macro;

#[proc_macro_derive(Repository, attributes(entity, id, table))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    postgres_macro::repository_derive(input)
}