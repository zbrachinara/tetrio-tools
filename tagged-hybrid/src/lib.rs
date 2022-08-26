#![feature(let_else)]

use std::collections::{HashMap, HashSet};

use itertools::{izip, Itertools};
use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, FieldsNamed};
use tap::{Pipe, Tap};
use heck::ToSnakeCase;

#[proc_macro_attribute]
pub fn hybrid_tagged(attr: TokenStream, item: TokenStream) -> TokenStream {
    hybrid_tagged_impl(attr.into(), item.into()).into()
    // let ret = hybrid_tagged_impl(attr.into(), item.into()).into();
    // println!("{ret}");
    // ret
}

fn hybrid_tagged_impl(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let tagged_type: DeriveInput = syn::parse2(item).unwrap();
    let Data::Enum(tagged_enum) = tagged_type.data else {
        panic!("hybrid_tagged is meant to be invoked on an enum")
    };

    let args = attr_args(attr);

    let common_fields = args
            .get("fields")
            .expect("Argument `fields` was not provided")
            .pipe(|tokens| {
                syn::parse2::<FieldsNamed>(tokens.to_token_stream())
                    .expect("Fields should be written with the same notation as a struct declaration, inside curly braces")
            });
    let common_fields_inner = &common_fields.named;
    let tag = args
        .get("tag")
        .expect("Argument `tag` was not provided")
        .to_token_stream();
    let variants = tagged_enum.variants;
    // whichever variants we do not have object data to collect for
    let empty_variants = variants
        .iter()
        .cloned()
        .filter(|variant| matches!(variant.fields, Fields::Unit))
        .map(|variant| variant.ident)
        .collect::<HashSet<_>>();

    let original_attrs = tagged_type.attrs;

    let visibility = tagged_type.vis;

    // takes the variants of the annotated enum and adds the common fields to each one
    let raw_variants = variants.clone().tap_mut(|variants| {
        variants.iter_mut().for_each(|variant| {
            let attrs = &variant.attrs;
            let name = &variant.ident;
            let common_fields = common_fields_inner.iter();

            *variant = if empty_variants.contains(&variant.ident) {
                syn::parse_quote!(
                    #(#attrs)*
                    #name {
                        #(#common_fields),*
                    }
                )
            } else {
                syn::parse_quote!(
                    #(#attrs)*
                    #name {
                        data: #name,
                        #(#common_fields),*
                    }
                )
            };
        })
    });

    let data_variants = variants.clone().tap_mut(|variants| {
        variants.iter_mut().for_each(|variant| {
            variant.attrs.clear();
            match variant.fields {
                Fields::Named(ref mut f) => {
                    for field in &mut f.named {
                        field.attrs.clear();
                    }
                }
                _ => (),
            }
        })
    });

    let struct_attrs = args.get("struct_attrs").map(|tokens| {
        syn::parse2::<Group>(tokens.into_token_stream())
            .unwrap()
            .stream()
    });

    // raw enum which serde directly translates from json
    let raw_enum = quote!(
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(tag=#tag)]
        #(#original_attrs)*
        enum Raw {
            #raw_variants
        }
    );

    // data enum containing the specific data for each variant
    let data_enum = quote!(
        #[derive(Clone)]
        #struct_attrs
        #visibility enum Variant {
            #data_variants
        }
    );

    // public-facing struct which takes the place of the annotated enum
    let public_struct = quote!(
        #[derive(serde::Serialize, serde::Deserialize, Clone)]
        #[serde(from = "Raw", into = "Raw")]
        #struct_attrs
        pub struct Container {
            data: Variant,
            #common_fields_inner
        }
    );

    let common_fields_names = common_fields_inner
        .iter()
        .cloned()
        .map(|field| field.ident.expect("Fields of this enum must be named"))
        .collect_vec();
    let common_fields_renamed = common_fields_names.iter().cloned().map(|name| {
        syn::parse_str::<Ident>(&format!("c_{name}")).unwrap()
    }).collect_vec(); // TODO: Make these names hygienic
    let variant_fields_names = variants
        .iter()
        .map(|variant| {
            variant
                .fields
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let raw_fields_names = raw_variants
        .iter()
        .map(|variant| {
            variant
                .fields
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let variant_names = variants
        .iter()
        .cloned()
        .map(|variant| variant.ident)
        .collect_vec();

    let variant_structs = variants.iter().map(|variant| {
            let name = &variant.ident;
            let fields = &variant.fields;

            if empty_variants.contains(&variant.ident) {
                quote!( #[derive(serde::Serialize, serde::Deserialize)] #struct_attrs struct #name; )
            } else {
                quote!( #[derive(serde::Serialize, serde::Deserialize)] #struct_attrs struct #name #fields )
            }
        });

    let (convert_from_raw, convert_to_raw): (Vec<_>, Vec<_>) =
        izip!(variant_fields_names, raw_fields_names)
            .zip(variant_names)
            .map(|((variant, _), variant_name)| {
                if empty_variants.contains(&variant_name) {
                    let from_raw = quote!(
                        Raw :: #variant_name {
                            #(#common_fields_names: #common_fields_renamed),*, ..
                        } => Self {
                            data: Variant:: #variant_name ,
                            #(#common_fields_names: #common_fields_renamed),*
                        }
                    );

                    let to_raw = quote!(
                        Variant :: #variant_name => Self :: #variant_name {
                            #(#common_fields_names: f. #common_fields_names)*,
                        }
                    );

                    (from_raw, to_raw)
                } else {
                    let from_raw = quote!(
                        Raw :: #variant_name {
                            data: #variant_name {
                                #(#variant),*
                            },
                            #(#common_fields_names: #common_fields_renamed),*
                        } => Self {
                            data: Variant :: #variant_name {
                                #(#variant),*
                            }, #(#common_fields_names: #common_fields_renamed),*
                        }
                    );

                    let to_raw = quote!(
                        Variant :: #variant_name {
                            #(#variant),*
                        } => Self :: #variant_name {
                            data: #variant_name {
                                #(#variant),*
                            },
                            #(#common_fields_names: f. #common_fields_names),*
                        }
                    );

                    (from_raw, to_raw)
                }
            })
            .unzip();

    // From impls for converting to and from the public struct and private type
    let convert_impls = quote!(
        impl From<Raw> for Container {
            fn from(f: Raw) -> Self {
                match f {
                    #(#convert_from_raw),*
                }
            }
        }

        impl From<Container> for Raw {
            fn from(f: Container) -> Self {
                match f.data {
                    #(#convert_to_raw),*
                }
            }
        }
    );

    let container_name = tagged_type.ident;
    let module_name = Ident::new(
        &format!("{}_data", container_name.to_string().to_snake_case()),
        container_name.span(),
    );
    let data_enum_name = Ident::new(&format!("{container_name}Data"), container_name.span());

    // all put together
    quote!(
        #visibility use #module_name::{
            Container as #container_name,
            Variant as #data_enum_name
        };
        mod #module_name {
            use super::*;
            #public_struct
            #raw_enum
            #data_enum

            #(#variant_structs)*

            #convert_impls
        }
    )
}

fn attr_args(attr: TokenStream2) -> HashMap<String, TokenTree> {
    attr.into_iter()
        .group_by(|tk| !matches!(tk, TokenTree::Punct(p) if p.as_char() == ','))
        .into_iter()
        .filter_map(|(cond, c)| cond.then(|| c))
        .map(|mut triple| {
            let ident = triple.next();
            let eq_sign = triple.next();
            let value = triple.next();

            if !matches!(eq_sign, Some(TokenTree::Punct(eq_sign)) if eq_sign.as_char() == '=') {
                panic!(r#"Attribute arguments should be in the form of `key = value`"#)
            }

            match (ident, value) {
                (Some(TokenTree::Ident(ident)), Some(value)) => (ident.to_string(), value),
                _ => panic!(r#"Attribute arguments should be in the form of `key = "value"`"#),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::hybrid_tagged_impl;
    use quote::quote;

    #[test]
    fn test_hybrid_tagged_impl() {
        let macro_out = hybrid_tagged_impl(
            quote!(tag = "type", fields = {frame: Number, slack: Slack,}, struct_attrs = {
                #[derive(Debug)]
                #[serde(rename = "UPPERCASE")]
            }),
            quote!(
                #[derive(Debug)]
                #[serde(some_other_thing)]
                pub(super) enum Variations {
                    A {
                        #[field_attribute]
                        task: T,
                        time: U,
                    },
                    B {
                        hours: H,
                        intervals: I,
                    },
                    HasFrame {
                        frame: F,
                    },
                    C,
                    // D(Wrong)
                }
            ),
        );

        println!("{}", macro_out)
    }
}
