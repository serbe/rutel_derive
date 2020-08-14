use proc_macro2::{Ident, Span, TokenStream};
use syn::{Meta};
use quote::quote;

fn to_snake_case(ident: &Ident) -> Ident {
    let input = ident.to_string();
    let new_name = input.chars().enumerate().fold(String::new(), |mut acc, (i, x)| {
        if x.is_uppercase() {
            if i != 0 { acc.push('_') };
            acc.push(x.to_ascii_lowercase());
        } else {
            acc.push(x);
        }
        acc
    });
    Ident::new(&new_name, Span::call_site())
}

fn params(ident: &Ident) -> Ident {
    let input = ident.to_string();
    let (start, end) = input.split_at(1);
    Ident::new(&format!("{}{}", start.to_lowercase(), end), Span::call_site())
}

fn get_attribute_type_multiple(
    ast: &syn::DeriveInput,
) -> Option<Vec<Option<syn::Type>>> {
    for attr in ast.attrs.clone() {
        let a = attr.parse_meta();
        match a {
            Ok(meta) => {
                eprintln!("meta ok");
                match meta {
                    Meta::Path(path) => {eprintln!("path {:?}", path.get_ident());}
                    Meta::List(list) => {eprintln!("metalist {:?}", list.path.get_ident());}
                    Meta::NameValue(name) => {eprintln!("metanamevalue {:?}", name.path.get_ident());}
                }
            },
            _ => (),
        }
    }
    // let attr = ast.attrs.iter().find(|a| {
    //     let a = a.parse_meta();
    //     match a {
    //         // Ok(ref meta) if meta.t() == "response" => true,
    //         Ok(syn::Meta::Path(path)) => {
    //             eprintln!("path {:?}", path.get_ident());
    //             true
    //         },
    //         _ => false,
    //     }
    // })?;
    // let attr = attr.interpret_meta()?;

    // if let syn::Meta::List(ref list) = attr {
    //     Some(
    //         list.nested
    //             .iter()
    //             .map(|m| meta_item_to_ty(m, "response"))
    //             .collect(),
    //     )
    // } else {
    //     panic!("The correct syntax is #[{}(type, type, ...)]", "response");
    // }
    None
}

pub fn parse(ast: &syn::DeriveInput) -> TokenStream {
    let name_fn = to_snake_case(&ast.ident);
    let name_params = params(&ast.ident);
    let _ = get_attribute_type_multiple(&ast);

    eprintln!("name_fn {}", name_fn);
    eprintln!("name_params {}", name_params);
    quote!{}
}