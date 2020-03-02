extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// mod ex;

#[proc_macro_attribute]
pub fn response(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(item as syn::ItemFn);

    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item

    // let name = &input.ident;
    // let expanded = ex::parse(&input);

    // TokenStream::from(expanded)

    // ex::parse(&ast).into()
}
