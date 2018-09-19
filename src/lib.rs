extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;

#[macro_use]
extern crate quote;

use syn::DeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{Data, Fields, Field, Type};

#[proc_macro_derive(GetSet)]
pub fn derive_get_set(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let gen = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => {
                let f: Vec<&Field> = fields.named.iter().filter(|f| f.ident.is_some()).collect();
                let all_fields: Vec<(&Field, bool)> = f.iter().map(|x| (*x, match x.ty {
                    Type::Path(ref p) => p.path.segments[0].ident == "Option",
                    _ => false,
                })).collect();
                let field_names: Vec<Ident> = all_fields.iter().map(|x| x.0.ident.clone().unwrap()).collect();
                let field_names2 = field_names.clone();
                let field_names_opt: Vec<Ident> = all_fields.iter().filter(|x| x.1).map(|x| x.0.ident.clone().unwrap()).collect();
                let field_names_no_opt: Vec<Ident> = all_fields.iter().filter(|x| !x.1).map(|x| x.0.ident.clone().unwrap()).collect();
                let field_names_no_opt2 = field_names_no_opt.clone();
                let getter_names: Vec<Ident> = field_names.iter().map(|x|
                    Ident::new(format!("get_{}", x).as_str(), Span::call_site())).collect();
                let setter_names: Vec<Ident> = field_names.iter().map(|x|
                    Ident::new(format!("{}", x).as_str(), Span::call_site())).collect();
                let field_types: Vec<Type> = all_fields.iter().map(|x| x.0.ty.clone()).collect();
                let field_types2 = field_types.clone();
                let field_types_no_opt: Vec<Type> = all_fields.iter().filter(|x| !x.1).map(|x| x.0.ty.clone()).collect();
                quote! {
                    // #[allow(dead_code)]
                    impl #name {
                        pub fn new(#(#field_names_no_opt: #field_types_no_opt,)*) -> Self {
                            #name{
                                #(#field_names_no_opt2,)*
                                #(#field_names_opt: None,)*
                            }
                        }

                        #(
                            pub fn #getter_names(&self) -> &#field_types {
                                &self.#field_names
                            }

                            pub fn #setter_names(&mut self, x : #field_types2) -> &mut Self {
                                self.#field_names2 = x;
                                self
                            }
                        )*

                        pub fn to_string(&self) -> String {
                            to_string(self).unwrap()
                        }
                    }
                }
            }
            _ => quote!{},
        },
        _ => quote!{},
    };

    println!("{:?}", gen);
    gen.into()
}
