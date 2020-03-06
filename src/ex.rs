use proc_macro2::{Ident, Span, TokenStream};
// use syn::{Data};
use quote::quote;
use syn::{Lit, LitStr, Meta, NestedMeta, ItemStruct, Fields, FieldsNamed};

pub const MESSAGE_ATTR: &str = "result";

pub fn result_type(args: Vec<syn::NestedMeta>) -> String {
    if args.len() != 2 {
        panic!("wrong attributes count");
    }

    match &args[0] {
        NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
            Some(i) if i.to_string() == MESSAGE_ATTR => (),
            _ => panic!("need some path \"result\""),
        },
        _ => panic!("need path \"result\""),
    }

    match &args[1] {
        NestedMeta::Lit(Lit::Str(lit_str)) => lit_str.value(),
        _ => panic!("need str"),
    }
}

pub fn parse_struct(input: syn::ItemStruct, r_type: String) -> TokenStream {
    println!("{}", r_type);
    let name_fn = to_snake_case(&input.ident);
    
    let body = if let Fields::Named(fields_named) = &input.fields {
        
        let f: Vec<&syn::Field> = fields_named.named.iter().filter(|f| f.ident.is_some()).collect();
        
        println!("len f {}", &f.len());

        let all_fields: Vec<(&syn::Field, bool)> = f.iter().map(|x| (*x, match x.ty {
                syn::Type::Path(ref p) => p.path.segments[0].ident == "Option",
                _ => false,
            })).collect();

            println!("len all f {}", &all_fields.len());

        // let mut f = Vec::new();


        // for field in &fields_named.named {
        //     // Get the field identifier
        //     let field_ident = field.ident.as_ref().unwrap();

        //     // Create the string literal for the Field constructor
        //     let field_name = field_ident.to_string();
        //     let field_type = &field.ty;
        //     // let _field_string = LitStr::new(&field_name, Span::call_site());

        //     // dbg!(field_name);
        //     // dbg!(const_ident);
        //     // dbg!(&field_type);
        //     // dbg!(&field_string);

        //     // Create the statement
        //     f.push(quote! {
        //         pub #field_name: #field_type;
        //     });
        // }

        // dbg!(&f);

        quote! {
            // #(#f)*
        }


        // quote! {
        //     // impl #struct_ident {
        //     //     #consts
        //     // }
        // }
    } else {
        panic!("The derive(Record) macro can only be used on structs with named fields")
    };

    // println!("{:?}", input.fields);
    // let item_type = {
    //     match get_attribute_type_multiple(ast, MESSAGE_ATTR) {
    //         Some(ty) => match ty.len() {
    //             1 => ty[0].clone(),
    //             _ => panic!(
    //                 "#[{}(type)] takes 1 parameters, given {}",
    //                 MESSAGE_ATTR,
    //                 ty.len()
    //             ),
    //         },
    //         None => None,
    //     }
    // };

    // let name = &ast.ident;
    // let name_fn = to_snake_case(&format!("{}", name));
    // let name_request = params(&format!("{}", name));

    // let item_type = match item_type {
    //     Some(ty) => quote!{ #ty },
    //     None => quote!{ () },
    // };

    // let fields = match input.fields {
    //         Fields::Named(ref fields) => fields,
    //         _ => panic!("data.fields"),
    // };

    // dbg!(input.fields.len());

    // let f: Vec<&syn::Field> = fields.named.iter().filter(|f| f.ident.is_some()).collect();
    // let all_fields: Vec<(&syn::Field, bool)> = f.iter().map(|x| (*x, match x.ty {
    //     syn::Type::Path(ref p) => p.path.segments[0].ident == "Option",
    //     _ => false,
    // })).collect();
    // let field_names: Vec<Ident> = all_fields.iter().map(|x| x.0.ident.clone().unwrap()).collect();
    // let field_names2 = field_names.clone();
    // let field_names_opt: Vec<Ident> = all_fields.iter().filter(|x| x.1).map(|x| x.0.ident.clone().unwrap()).collect();
    // let field_names_no_opt: Vec<Ident> = all_fields.iter().filter(|x| !x.1).map(|x| x.0.ident.clone().unwrap()).collect();
    // let field_names_no_opt2 = field_names_no_opt.clone();
    // let getter_names: Vec<Ident> = field_names.iter().map(|x|
    //     Ident::new(format!("get_{}", x).as_str(), Span::call_site())).collect();
    // let setter_names: Vec<Ident> = field_names.iter().map(|x|
    //     Ident::new(format!("{}", x).as_str(), Span::call_site())).collect();
    // let field_types: Vec<syn::Type> = all_fields.iter().map(|x| x.0.ty.clone()).collect();
    // let field_types2 = field_types.clone();
    // let field_types_no_opt: Vec<syn::Type> = all_fields.iter().filter(|x| !x.1).map(|x| x.0.ty.clone()).collect();

    // quote!{
    //     impl Bot {
    //         pub fn #name_fn(&mut self, v: &mut #name) -> Result<#item_type, String> {
    //             let resp = self.create_request(stringify!(#name_request), v.to_string())?;
    //             from_value(resp).map_err(|e| e.to_string())
    //         }
    //     }

    //     // #[allow(dead_code)]
    //     impl #name {
    //         pub fn new(#(#field_names_no_opt: #field_types_no_opt,)*) -> Self {
    //             #name{
    //                 #(#field_names_no_opt2,)*
    //                 #(#field_names_opt: None,)*
    //             }
    //         }

    //         #(
    //             pub fn #getter_names(&self) -> &#field_types {
    //                 &self.#field_names
    //             }

    //             pub fn #setter_names(&mut self, x : #field_types2) -> &mut Self {
    //                 self.#field_names2 = x;
    //                 self
    //             }
    //         )*

    //         pub fn to_string(&self) -> String {
    //             to_string(self).unwrap()
    //         }
    //     }
    // }
    quote! {
        // #struct_impl
        // impl Bot {
            pub fn #name_fn() {}
        // }
    }
}

// fn get_attribute_type_multiple(
//     ast: &syn::DeriveInput, name: &str,
// ) -> Option<Vec<Option<syn::Type>>> {
//     let attr = ast.attrs.iter().find(|a| {
//         let a = a.interpret_meta();
//         match a {
//             Some(ref meta) if meta.name() == name => true,
//             _ => false,
//         }
//     })?;
//     let attr = attr.interpret_meta()?;

//     if let syn::Meta::List(ref list) = attr {
//         Some(
//             list.nested
//                 .iter()
//                 .map(|m| meta_item_to_ty(m, name))
//                 .collect(),
//         )
//     } else {
//         panic!("The correct syntax is #[{}(type, type, ...)]", name);
//     }
// }

// fn meta_item_to_ty(meta_item: &NestedMeta, name: &str) -> Option<syn::Type> {
//     match *meta_item {
//         NestedMeta::Meta(syn::Meta::Word(ref i)) => {
//             if let Ok(ty) = syn::parse_str::<syn::Type>(&i.to_string()) {
//                 Some(ty)
//             } else {
//                 panic!("The correct syntax is #[{}(type)]", name);
//             }
//         }
//         NestedMeta::Meta(syn::Meta::NameValue(ref val)) => {
//             if val.ident == "result" {
//                 if let Lit::Str(ref s) = val.lit {
//                     if let Ok(ty) = syn::parse_str::<syn::Type>(&s.value().to_string()) {
//                         return Some(ty);
//                     } else {
//                         panic!("The correct syntax is #[{}(type)]", name);
//                     }
//                 }
//             }
//             panic!("The correct syntax is #[{}(result=\"TYPE\")]", name);
//         }
//         NestedMeta::Literal(Lit::Str(ref s)) => {
//             if let Ok(ty) = syn::parse_str::<syn::Type>(&s.value().to_string()) {
//                 return Some(ty);
//             } else {
//                 panic!("The correct syntax is #[{}(type)]", name);
//             }
//         }
//         _ => panic!("The correct syntax is #[{}(type)]", name),
//     }
// }

fn to_snake_case(input: &Ident) -> Ident {
    let new_name = input
        .to_string()
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

// fn params(input: &str) -> Ident {
//     let (start, end) = input.split_at(1);
//     Ident::new(&format!("{}{}", start.to_lowercase(), end), Span::call_site())
// }
