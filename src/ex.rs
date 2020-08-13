use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, AttributeArgs, Ident, NestedMeta};

struct Args {
    path: syn::LitStr,
    guards: Vec<Ident>,
    wrappers: Vec<syn::Type>,
}

impl Args {
    fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut path = None;
        let mut guards = Vec::new();
        let mut wrappers = Vec::new();
        for arg in args {
            match arg {
                NestedMeta::Lit(syn::Lit::Str(lit)) => match path {
                    None => {
                        path = Some(lit);
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            lit,
                            "Multiple paths specified! Should be only one!",
                        ));
                    }
                },
                NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.is_ident("guard") {
                        if let syn::Lit::Str(lit) = nv.lit {
                            guards.push(Ident::new(&lit.value(), Span::call_site()));
                        } else {
                            return Err(syn::Error::new_spanned(
                                nv.lit,
                                "Attribute guard expects literal string!",
                            ));
                        }
                    } else if nv.path.is_ident("wrap") {
                        if let syn::Lit::Str(lit) = nv.lit {
                            wrappers.push(lit.parse()?);
                        } else {
                            return Err(syn::Error::new_spanned(
                                nv.lit,
                                "Attribute guard expects type",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            nv.path,
                            "Unknown attribute key is specified. Allowed: guard and wrap",
                        ));
                    }
                }
                arg => {
                    return Err(syn::Error::new_spanned(arg, "Unknown attribute."));
                }
            }
        }
        Ok(Args {
            path: path.unwrap(),
            guards,
            wrappers,
        })
    }
}

pub struct Response {
    name: syn::Ident,
    args: Args,
    ast: syn::ItemFn,
}

impl Response {
    pub fn new(args: AttributeArgs, input: TokenStream) -> syn::Result<Self> {
        if args.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                String::from("invalid server definition"),
            ));
        }
        let ast: syn::ItemFn = syn::parse(input)?;
        let name = ast.sig.ident.clone();

        let args = Args::new(args)?;

        Ok(Self { name, args, ast })
    }
}

impl ToTokens for Response {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let Self {
            name,
            ast,
            args:
                Args {
                    path,
                    guards,
                    wrappers,
                },
        } = self;
        let resource_name = name.to_string();
        let stream = quote! {
            #[allow(non_camel_case_types, missing_docs)]
            pub struct #name;

            // impl actix_web::dev::HttpServiceFactory for #name {
            //     fn register(self, __config: &mut actix_web::dev::AppService) {
            //         #ast
            //         let __resource = actix_web::Resource::new(#path)
            //             .name(#resource_name)
            //             .guard(actix_web::guard::#guard())
            //             #(.guard(actix_web::guard::fn_guard(#guards)))*
            //             #(.wrap(#wrappers))*
            //             .#resource_type(#name);

            //         actix_web::dev::HttpServiceFactory::register(__resource, __config)
            //     }
            // }
        };

        output.extend(stream);
    }
}

pub(crate) fn generate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    match Response::new(args, input) {
        Ok(response) => response.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
