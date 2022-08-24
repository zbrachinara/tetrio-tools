#![feature(let_else)]

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, FieldsNamed};
use tap::{Pipe, Tap};
// use venial::{Declaration, StructFields, NamedStructFields};

#[proc_macro_attribute]
pub fn hybrid_tagged(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr}");
    println!("{item}");

    hybrid_tagged_impl(attr.into(), item.into()).into()
}

fn hybrid_tagged_impl(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let tagged_type: DeriveInput = syn::parse2(item).unwrap();
    let Data::Enum(tagged_enum) = tagged_type.data else {
        panic!("hybrid_tagged is meant to be invoked on an enum")
    };

    let args = attr_args(attr);
    let common_fields = args
        .get("fields")
        .expect("Argument `fields` was not provided to the proc macro")
        .pipe(|tokens| syn::parse2::<FieldsNamed>(tokens.to_token_stream()).unwrap());

    let ret = {
        let tag = args.get("tag").unwrap().to_token_stream();
        let tagged_type_variants = tagged_enum.variants;
        let name = tagged_type.ident;
        let module_name = Ident::new(
            &name
                .to_string()
                .to_lowercase()
                .tap_mut(|s| s.push_str("_data")),
            name.span(),
        );
        let raw_name = Ident::new(&format!("Raw{name}"), name.span());
        let original_attrs = tagged_type.attrs;

        let raw_variants = tagged_type_variants.clone().tap_mut(|variants| {
            variants
                .iter_mut()
                .for_each(|variant| match variant.fields {
                    Fields::Named(ref mut f) => common_fields
                        .named
                        .iter()
                        .cloned()
                        .for_each(|field| f.named.push(field)),
                    Fields::Unit => variant.fields = Fields::Named(common_fields.clone()),
                    _ => panic!("Fields of this enum must be named"),
                })
        });
        let raw_enum = quote!(
            #[serde(tag=#tag)] #(#original_attrs)* enum #raw_name {
                #raw_variants
            }
        );
        let public_struct = quote!(
            struct F {
                u: U,
            }
        );

        quote!(
            pub #public_struct

            mod #module_name {
                #raw_enum
            }
        )
    };

    println!("{ret}");

    ret
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
        hybrid_tagged_impl(
            quote!(tag = "type", fields = {frame: Number, slack: Swick}),
            quote!(
                #[Derive(Serialize, Deserialize)]
                #[serde(some_other_thing)]
                enum Variations {
                    A { task: T, time: U },
                    B { hours: H, intervals: I },
                    C,
                    // D(Wrong)
                }
            ),
        );
    }
}
