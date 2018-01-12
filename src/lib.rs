#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

#![feature(plugin)]
#![plugin(clippy)]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::BTreeMap;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(GetSet)]
pub fn qqq(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let _attributes = get_attributes(&ast.attrs);
    // let method = attrs.get(&"method".to_string()).unwrap();
    // println!("{}", method);
    let struct_name = &ast.ident;
    if let syn::Body::Struct(s) = ast.body {
        let field_names: Vec<_> = s.fields().iter().map(|x| x.ident.clone().unwrap()).collect();
        let field_names_opt: Vec<_> = s.fields().iter().filter(|x| {
            match x.ty.clone() {
                syn::Ty::Path(_, path) => "Option" == format!("{}", path.segments[0].ident),
                _ => false,
            }
        }).map(|x| x.ident.clone().unwrap()).collect();
        // let field_names_no_opt: Vec<_> = s.fields().iter().filter(|ref x| {
        //     match x.ty.clone() {
        //         syn::Ty::Path(_, path) => "Option" != format!("{}", path.segments[0].ident),
        //         _ => false,
        //     }
        // }).map(|ref x| x.ident.clone().unwrap()).collect();
        let field_getter_names = field_names.iter().map(|x|
            syn::Ident::new(format!("get_{}", x).as_str()));
        let field_setter_names = field_names.iter().map(|x|
            syn::Ident::new(format!("set_{}", x).as_str()));
        let field_setter_opt_names = field_names_opt.iter().map(|x|
            syn::Ident::new(format!("{}", x).as_str()));
        let field_types: Vec<_> = s.fields().iter().map(|x| x.ty.clone()).collect();
        let field_types_opt: Vec<_> = s.fields().iter().filter(|x| {
            match x.ty.clone() {
                syn::Ty::Path(_, path) => "Option" == format!("{}", path.segments[0].ident),
                _ => false,
            }
        }).map(|x| x.ty.clone()).collect();
        // let field_types_no_opt: Vec<_> = s.fields().iter().filter(|ref x| {
        //     match x.ty.clone() {
        //         syn::Ty::Path(_, path) => "Option" != format!("{}", path.segments[0].ident),
        //         _ => false,
        //     }
        // }).map(|ref x| x.ty.clone()).collect();
        let field_names2 = field_names.clone();
        let field_names3 = field_names.clone();
        let field_types2 = field_types.clone();
        let field_names_opt2 = field_names_opt.clone();
        // let struct_name2 = struct_name.clone();
        // let struct_name3 = struct_name.clone();
        // let field_names_no_opt2 = field_names_no_opt.clone();
        // let field_types_no_opt2 = field_types_no_opt.clone();

        let quoted_code = quote! {
            #[allow(dead_code)]
            impl #struct_name {
                #(
//                    pub fn new2(#( #field_names_no_opt: #field_types_no_opt, )*) -> #struct_name2 {
//                        #struct_name3 {
//                            #( #field_names_no_opt2: #field_types_no_opt2, )*
//                        }
//                    }

                    pub fn #field_getter_names(&self) -> &#field_types {
                        &self.#field_names2
                    }
                    pub fn #field_setter_names(&mut self, x : #field_types2) -> &mut Self {
                        self.#field_names3 = x;
                        self
                    }
                    pub fn #field_setter_opt_names(&mut self, x : #field_types_opt) -> &mut Self {
                        self.#field_names_opt2 = x;
                        self
                    }
                )*

                // pub fn method(&self) -> String {
                //     #method
                // }

                pub fn json(&self) -> String {
                    to_string(self).unwrap()
                }
            }
        };
        return quoted_code.parse().unwrap();
    }
    // not a struct
    "".parse().unwrap()
}

fn get_attributes(attributes: &[syn::Attribute]) -> BTreeMap<String, String> {
    attributes.iter().fold(BTreeMap::new(), |mut btm, attr| {
        if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
            if let syn::Lit::Str(val, _) = value.clone() {
                btm.insert(name.to_string(), val.clone());
            };
        };
        btm
    })
}

