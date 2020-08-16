use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ex;

// #[proc_macro_attribute]
// pub fn generate(attr: TokenStream, _item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(attr as DeriveInput);
//     // let item = input.clone();
//     let name = dbg!(input).ident;
//     eprintln!("name: {}", name);

//     // let _ = ex::parse(&input);

//     quote!("").into()
//     // ex::generate(args, input)
// }

#[proc_macro_derive(Response, attributes(response))]
pub fn response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // dbg!(input.clone());

    let token = ex::parse(&input);

    dbg!(&token.to_string());

    token.into()
}

// https://doc.rust-lang.org/stable/reference/macros-by-example.html
// https://doc.rust-lang.org/stable/reference/procedural-macros.html
// https://doc.rust-lang.org/stable/reference/attributes.html
// https://cheats.rs/#macros-attributes
// https://www.thetopsites.net/article/59429942.shtml
// https://danielkeep.github.io/tlborm/book/pat-visibility.html
