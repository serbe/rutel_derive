extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod ex;

#[proc_macro_derive(GetSet, attributes(response))]
pub fn derive_derive_get_set_response(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    ex::parse(&ast).into()
}
