// #![crate_type = "proc-macro"]
//#![recursion_limit = "192"]
// #![feature(proc_macro, proc_macro_lib)]
//#![feature(plugin)]
//#![plugin(clippy)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, DeriveInput, Ident, parse_derive_input, Ty};
// use std::collections::BTreeMap;

#[proc_macro_derive(GetSet)]
pub fn derive_get_set(input: TokenStream) -> TokenStream {
    let ast = into_ast(&input);
    // println!("{:?}", attrs["func"]);
    // let method = match attrs.get("method") {
    //     Some(v) => v,
    //     _ => "",
    // };
    let struct_name = &ast.ident;
    let struct_name2 = struct_name.clone();
    if let Body::Struct(s) = ast.body {
        let field_names: Vec<_> = s.fields().iter().map(|x| x.ident.clone().unwrap()).collect();
        let field_names_opt: Vec<_> = s.fields().iter().filter(|x| {
            match x.ty.clone() {
                Ty::Path(_, path) => "Option" == path.segments[0].ident.as_ref(),
                _ => false,
            }
        }).map(|x| x.ident.clone().unwrap()).collect();
        let field_names_no_opt: Vec<_> = s.fields().iter().filter(|x| {
            match x.ty.clone() {
                Ty::Path(_, path) => "Option" != path.segments[0].ident.as_ref(),
                _ => false,
            }
        }).map(|x| x.ident.clone().unwrap()).collect();
        let field_getter_names = field_names.iter().map(|x|
            Ident::new(format!("get_{}", x).as_str()));
        let field_setter_names = field_names.iter().map(|x|
            Ident::new(format!("{}", x).as_str()));
        let field_types: Vec<_> = s.fields().iter().map(|x| x.ty.clone()).collect();
        let field_types_no_opt: Vec<_> = s.fields().iter().filter(|x| {
            match x.ty.clone() {
                Ty::Path(_, path) => "Option" != path.segments[0].ident.as_ref(),
                _ => false,
            }
        }).map(|x| x.ty.clone()).collect();
        let field_names_no_opt2 = field_names_no_opt.clone();
        let field_names_opt3 = field_names_opt.clone();
        let field_names2 = field_names.clone();
        let field_names3 = field_names.clone();
        let field_types2 = field_types.clone();
        let quoted_code = quote! {
            // #[allow(dead_code)]
            impl #struct_name {
                pub fn new(#(#field_names_no_opt: #field_types_no_opt,)*) -> Self {
                    #struct_name2{
                        #(#field_names_no_opt2,)*
                        #(#field_names_opt3: None,)*
                    }
                }

                #(
                    pub fn #field_getter_names(&self) -> &#field_types {
                        &self.#field_names2
                    }

                    pub fn #field_setter_names(&mut self, x : #field_types2) -> &mut Self {
                        self.#field_names3 = x;
                        self
                    }
                )*

//                pub fn method(&self) -> String {
//                    #method.to_string()
//                }

                pub fn to_string(&self) -> String {
                    to_string(self).unwrap()
                }
            }

//            impl Bot {
//                pub fn get_updates(&mut self, v: &mut params::GetUpdatesParams) -> Result<Vec<Update>, String> {
//                    let resp = self.create_request("getUpdates", v.to_string())?;
//                    from_value(resp).map_err(|e| e.to_string())
//                }
//            }
        };
        // println!("{:#?}", quoted_code);
        return quoted_code.parse().unwrap();
    }
    "".parse().unwrap()
}

fn into_ast(input: &TokenStream) -> DeriveInput {
    let source = input.to_string();
    parse_derive_input(&source).unwrap()
}

// fn get_attributes(attributes: &[syn::Attribute]) -> BTreeMap<String, String> {
//     attributes.iter().fold(BTreeMap::new(), |mut result, attr| {
//         if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
//             if let syn::Lit::Str(val, _) = value.clone() {
//                 println!("{} {}", name.to_string(), val.clone());
//                 result.insert(name.to_string(), val.clone());
//             };
//         };
//         result
//     })
// }

// fn to_snake_case(s: &str) -> String {
//     let mut result = String::new();
//     for (i, c) in s.char_indices() {
//         if i > 0 && c.is_uppercase() {
//             result.push('_');
//         }
//         result.push(c.to_ascii_lowercase());
//     }
//     result
// }
