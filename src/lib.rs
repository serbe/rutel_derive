extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ex;

#[proc_macro_derive(GetSet, attributes(response))]
pub fn derive_derive_get_set_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = ex::parse(&input);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)

    // ex::parse(&ast).into()
}
