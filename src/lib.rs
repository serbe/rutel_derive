use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod ex;

#[proc_macro_derive(Response, attributes(response))]
pub fn response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let _ = ex::parse(&input);

    quote!("// test").into()
    // ex::generate(args, input)
}

// https://doc.rust-lang.org/stable/reference/macros-by-example.html
// https://doc.rust-lang.org/stable/reference/procedural-macros.html
// https://doc.rust-lang.org/stable/reference/attributes.html
// https://cheats.rs/#macros-attributes
// https://www.thetopsites.net/article/59429942.shtml
// https://danielkeep.github.io/tlborm/book/pat-visibility.html