use proc_macro::TokenStream;

mod ex;

#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    ex::generate(args, input)
}
