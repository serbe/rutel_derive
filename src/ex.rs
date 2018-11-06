use proc_macro2::{Ident, Span, TokenStream};
use syn::{Data, Lit, NestedMeta};

pub const MESSAGE_ATTR: &str = "response";

pub fn parse(ast: &syn::DeriveInput) -> TokenStream {
    let item_type = {
        match get_attribute_type_multiple(ast, MESSAGE_ATTR) {
            Some(ty) => match ty.len() {
                1 => ty[0].clone(),
                _ => panic!(
                    "#[{}(type)] takes 1 parameters, given {}",
                    MESSAGE_ATTR,
                    ty.len()
                ),
            },
            None => None,
        }
    };

    let name = &ast.ident;
    let name_fn = to_snake_case(&format!("{}", name));
    let name_request = params(&format!("{}", name));

    let item_type = match item_type {
        Some(ty) => quote!{ #ty },
        None => quote!{ () },
    };

    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields,
            _ => panic!("data.fields"),
        },
        _ => panic!("ast.data"),
    };

    let f: Vec<&syn::Field> = fields.named.iter().filter(|f| f.ident.is_some()).collect();
    let all_fields: Vec<(&syn::Field, bool)> = f.iter().map(|x| (*x, match x.ty {
        syn::Type::Path(ref p) => p.path.segments[0].ident == "Option",
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
    let field_types: Vec<syn::Type> = all_fields.iter().map(|x| x.0.ty.clone()).collect();
    let field_types2 = field_types.clone();
    let field_types_no_opt: Vec<syn::Type> = all_fields.iter().filter(|x| !x.1).map(|x| x.0.ty.clone()).collect();

    quote!{
        impl Bot {
            pub fn #name_fn(&mut self, v: &mut #name) -> Result<#item_type, String> {
                let resp = self.create_request(stringify!(#name_request), v.to_str())?;
                from_value(resp).map_err(|e| e.to_string())
            }
        }

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
                &to_string(self).unwrap()
            }
        }
    }
}

fn get_attribute_type_multiple(
    ast: &syn::DeriveInput, name: &str,
) -> Option<Vec<Option<syn::Type>>> {
    let attr = ast.attrs.iter().find(|a| {
        let a = a.interpret_meta();
        match a {
            Some(ref meta) if meta.name() == name => true,
            _ => false,
        }
    })?;
    let attr = attr.interpret_meta()?;

    if let syn::Meta::List(ref list) = attr {
        Some(
            list.nested
                .iter()
                .map(|m| meta_item_to_ty(m, name))
                .collect(),
        )
    } else {
        panic!("The correct syntax is #[{}(type, type, ...)]", name);
    }
}

fn meta_item_to_ty(meta_item: &NestedMeta, name: &str) -> Option<syn::Type> {
    match *meta_item {
        NestedMeta::Meta(syn::Meta::Word(ref i)) => {
            if let Ok(ty) = syn::parse_str::<syn::Type>(&i.to_string()) {
                Some(ty)
            } else {
                panic!("The correct syntax is #[{}(type)]", name);
            }
        }
        NestedMeta::Meta(syn::Meta::NameValue(ref val)) => {
            if val.ident == "result" {
                if let Lit::Str(ref s) = val.lit {
                    if let Ok(ty) = syn::parse_str::<syn::Type>(&s.value().to_string()) {
                        return Some(ty);
                    } else {
                        panic!("The correct syntax is #[{}(type)]", name);
                    }
                }
            }
            panic!("The correct syntax is #[{}(result=\"TYPE\")]", name);
        }
        NestedMeta::Literal(Lit::Str(ref s)) => {
            if let Ok(ty) = syn::parse_str::<syn::Type>(&s.value().to_string()) {
                return Some(ty);
            } else {
                panic!("The correct syntax is #[{}(type)]", name);
            }
        }
        _ => panic!("The correct syntax is #[{}(type)]", name),
    }
}

fn to_snake_case(input: &str) -> Ident {
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

fn params(input: &str) -> Ident {
    let (start, end) = input.split_at(1);
    Ident::new(&format!("{}{}", start.to_lowercase(), end), Span::call_site())
}