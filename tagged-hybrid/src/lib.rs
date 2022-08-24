#![feature(let_else)]

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Punct, TokenStream as TokenStream2, TokenTree};
use venial::Declaration;

#[proc_macro_attribute]
pub fn hybrid_tagged(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr}");
    println!("{item}");

    hybrid_tagged_impl(attr.into(), item.into())
}

fn hybrid_tagged_impl(attr: TokenStream2, item: TokenStream2) -> TokenStream {
    let decl = venial::parse_declaration(item);

    let Ok(Declaration::Struct(typed_data)) = decl else {
        panic!("hybrid_tagged is meant to be invoked on a struct")
    };

    let ls = attr_list(attr);
    println!("{ls:?}");

    todo!()
}

fn attr_list(attr: TokenStream2) -> HashMap<String, Literal> {
    attr.into_iter()
        .group_by(|tk| !matches!(tk, TokenTree::Punct(p) if p.as_char() == ','))
        .into_iter()
        .filter_map(|(cond, c)| cond.then(|| c))
        .map(|mut triple| {
            let ident = triple.next();
            let eq_sign = triple.next();
            let lit = triple.next();

            assert!(matches!(eq_sign, Some(TokenTree::Punct(eq_sign)) if eq_sign.as_char() == '='));

            match (ident, lit) {
                (Some(TokenTree::Ident(ident)), Some(TokenTree::Literal(lit))) => (ident.to_string(), lit),
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
            quote!(tag = "type", variant = "varying"),
            quote!(
                struct S {
                    frame: Number,
                    slack: Swick,
                    varying: Variations,
                }
            ),
        );
    }
}
