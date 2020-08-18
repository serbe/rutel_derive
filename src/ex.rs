use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, Data, Error, Field, Lit, Meta, Type};

fn syn_err(message: &str) -> Error {
    Error::new(Span::call_site(), message)
}

fn to_snake_case(ident: &Ident) -> Ident {
    let input = ident.to_string();
    let snake_case_name = input
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
    Ident::new(&snake_case_name, Span::call_site())
}

fn params(ident: &Ident) -> Ident {
    let input = ident.to_string();
    let (start, end) = input.split_at(1);
    Ident::new(
        &format!("{}{}", start.to_lowercase(), end),
        Span::call_site(),
    )
}

fn message_type(attrs: &Vec<Attribute>) -> Result<Type, Error> {
    let attr = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("response"))
        .next()
        .ok_or(syn_err(
            "cannot find `response` attribute in target struct.",
        ))?;
    let meta = attr.parse_meta()?;
    let lit = match meta {
        Meta::List(_list) => Err(syn_err("list is no meta name value")),
        Meta::Path(_path) => Err(syn_err("path is no meta name value")),
        Meta::NameValue(value) => Ok(value.lit),
    }?;
    let value = match lit {
        Lit::Str(litstr) => Ok(litstr.value()),
        _ => Err(syn_err("wrong lit")),
    }?;
    let ty = syn::parse_str::<Type>(&value)?;
    Ok(ty)
}

fn fields(data: &Data) -> Result<Vec<(&Field, bool)>, Error> {
    let named_fields = match data {
        Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => Ok(fields),
            _ => Err(syn_err("wrong fields")),
        },
        _ => Err(syn_err("data not struct")),
    }?;
    let somed_fields: Vec<&Field> = named_fields
        .named
        .iter()
        .filter(|f| f.ident.is_some())
        .collect();
    let all_fields: Vec<(&Field, bool)> = somed_fields
        .iter()
        .map(|x| {
            (
                *x,
                match x.ty {
                    syn::Type::Path(ref p) => p.path.segments[0].ident == "Option",
                    _ => false,
                },
            )
        })
        .collect();
    Ok(all_fields)
}

fn impl_bot(name: &Ident, attrs: &Vec<Attribute>) -> TokenStream {
    let name_fn = to_snake_case(&name);
    let name_request = params(&name).to_string();
    let message_type = message_type(&attrs).unwrap();
    quote! {
        impl Bot {
            pub async fn #name_fn(&mut self, v: &mut #name) -> Result<#message_type> {
                let resp = self.create_request(#name_request, v.to_string()?).await?;
                Ok(from_value(resp)?)
            }
        }
    }
}

fn new_fn(name: &Ident, all_fields: &Vec<(&Field, bool)>) -> TokenStream {
    let field_names_opt: Vec<Ident> = all_fields
        .iter()
        .filter(|x| x.1)
        .filter_map(|x| x.0.ident.clone())
        .collect();
    let field_names_no_opt: Vec<Ident> = all_fields
        .iter()
        .filter(|x| !x.1)
        .filter_map(|x| x.0.ident.clone())
        .collect();
    let field_types_no_opt: Vec<syn::Type> = all_fields
        .iter()
        .filter(|x| !x.1)
        .map(|x| x.0.ty.clone())
        .collect();
    quote! {
        pub fn new(#(#field_names_no_opt: #field_types_no_opt,)*) -> Self {
            #name{
                #(#field_names_no_opt,)*
                #(#field_names_opt: None,)*
            }
        }
    }
}

fn getters(all_fields: &Vec<(&Field, bool)>) -> TokenStream {
    let field_names: &Vec<Ident> = &all_fields
        .iter()
        .filter_map(|x| x.0.ident.clone())
        .collect();
    let getter_names: &Vec<Ident> = &field_names
        .iter()
        .map(|x| Ident::new(format!("get_{}", x).as_str(), Span::call_site()))
        .collect();
    let field_types: &Vec<syn::Type> = &all_fields.iter().map(|x| x.0.ty.clone()).collect();
    quote! {
        #(
            pub fn #getter_names(&self) -> &#field_types {
                &self.#field_names
            }
        )*
    }
}

fn setters(all_fields: &Vec<(&Field, bool)>) -> TokenStream {
    let field_names: &Vec<Ident> = &all_fields
        .iter()
        .filter_map(|x| x.0.ident.clone())
        .collect();
    let setter_names: &Vec<Ident> = &field_names
        .iter()
        .map(|x| Ident::new(format!("{}", x).as_str(), Span::call_site()))
        .collect();
    let field_types: &Vec<syn::Type> = &all_fields.iter().map(|x| x.0.ty.clone()).collect();
    quote! {
        #(
            pub fn #setter_names(&mut self, x : #field_types) -> &mut Self {
                self.#field_names = x;
                self
            }
        )*
    }
}

pub fn parse(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let all_fields = fields(&ast.data).unwrap();

    let impl_bot_quote = impl_bot(&ast.ident, &ast.attrs);
    let new_quote = new_fn(&ast.ident, &all_fields);
    let getters_quote = getters(&all_fields);
    let setters_quote = setters(&all_fields);

    // dbg!(&impl_bot_quote.to_string());
    // dbg!(&getters_quote.to_string());
    // dbg!(&setters_quote.to_string());

    quote! {
        #impl_bot_quote

        impl #name {
            #new_quote

            #getters_quote

            #setters_quote

            pub fn to_string(&self) -> Result<String> {
                Ok(to_string(self)?)
            }
        }
    }
}
