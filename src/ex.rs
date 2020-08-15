use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Meta, Data, Attribute};

fn to_snake_case(ident: &Ident) -> Ident {
    let input = ident.to_string();
    let new_name = input
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, x)| {
            if x.is_uppercase() {
                if i != 0 {
                    acc.push('_')
                };
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
    Ident::new(
        &format!("{}{}", start.to_lowercase(), end),
        Span::call_site(),
    )
}

fn from_attr(attr: &Attribute) {
    eprintln!("Attribute path {:?}", attr.path.get_ident());
    if let Ok(meta) = attr.parse_meta() {
        let _ = match meta {
            Meta::Path(_) => {
                eprintln!("Found response without name or description.");
                return;
            },
            Meta::NameValue(_) => {
                eprintln!("Couldn't parse this parameter attribute.");
                return;
            },
            Meta::List(list) => {
                eprintln!("have Meta::List attribute");
                eprintln!("MetaList {:?}", list.path.get_ident())
            },
        };
    } else {
                    eprintln!("Could not parse `response` attribute");
    };
}

fn get_attribute_type_multiple(attrs: &Vec<Attribute>) -> Option<Vec<Option<syn::Type>>> {
    let mut filter_attrs = attrs.iter().filter(|attr| attr.path.is_ident("response"));
    match (filter_attrs.next(), filter_attrs.next()) {
        (Some(attr), None) => {eprintln!("Found one definitions for `response` attribute.");from_attr(attr)},
        (_, Some(_)) => eprintln!("Found multiple definitions for `response` attribute."),
        _ => eprintln!("Cannot find `response` attribute in target struct."),
    }
    // for attr in ast.attrs.clone() {
    //     let a = attr.parse_meta();
    //     eprintln!("attr path {:?}", attr.path.get_ident());
    //     if let Ok(parse_attr) = attr.parse_args::<Lit>() {
    //         match parse_attr {
    //             Lit::Bool(b) => {eprintln!("LitBool {}", b.value)}
    //             Lit::Str(s) => {eprintln!("LitStr {}", s.value())}
    //             Lit::ByteStr(bs) => {eprintln!("LitByteStr {:?}", bs.value())}
    //             Lit::Byte(b) => {eprintln!("LitByte {}", b.value())}
    //             Lit::Char(c) => {eprintln!("LitChar {}", c.value())}
    //             Lit::Int(i) => {eprintln!("LitInt {}", i.to_string())}
    //             Lit::Float(f) => {eprintln!("LitFloat {}", f.to_string())}
    //             Lit::Verbatim(v) => {eprintln!("LitVerbatim {}", v.to_string())}
    //         }
    //     } else {
    //         eprintln!("No Lit");
    //     };
    //     match a {
    //         Ok(meta) => {
    //             eprintln!("meta ok");
    //             match meta {
    //                 Meta::Path(path) => {
    //                     eprintln!("path {:?}", path.get_ident());
    //                 }
    //                 Meta::List(list) => {
    //                     eprintln!("metalist {:?}", list.path.get_ident());
    //                 }
    //                 Meta::NameValue(name) => {
    //                     eprintln!("metanamevalue {:?}", name.path.get_ident());
    //                 }
    //             }
    //         }
    //         _ => {
    //             eprintln!("meta err");
    //         }
    //     }
    // }
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

fn validate_data(data: &Data) {
    match data {
        Data::Struct(_) => eprintln!("have data struct"),
        Data::Enum(_) => eprintln!("have data enum"),
        Data::Union(_) => eprintln!("have data union"),
    }
}

pub fn parse(ast: &syn::DeriveInput) -> TokenStream {
    let name_fn = to_snake_case(&ast.ident);
    let name_params = params(&ast.ident);
    let _ = get_attribute_type_multiple(&ast.attrs);
    validate_data(&ast.data);

    eprintln!("name_fn {}", name_fn);
    eprintln!("name_params {}", name_params);
    quote! {}
}
