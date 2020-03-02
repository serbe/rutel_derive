extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
// #[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemStruct};

mod ex;

#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: \"{}\"", args.to_string());
    // println!("input: \"{}\"", input.to_string());

    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemStruct);

    let r_type = ex::result_type(args);
    println!("r_type: {}", r_type);

    // let input2 = parse_macro_input!(input as ItemFn);

    // let input = parse_macro_input!(item as syn::Item);
    // println!("args: \"{}\"", args.to_string());
    // println!("input2: \"{}\"", input2.to_string());

    let expanded = ex::parse_struct(input, r_type);

    proc_macro::TokenStream::from(expanded)

    // let name = &input.ident;

    // TokenStream::from(expanded)

    // ex::parse(&ast).into()
}
