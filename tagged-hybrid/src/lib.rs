#![feature(let_else)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use venial::Declaration;

#[proc_macro_attribute]
pub fn hybrid_tagged(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr}");
    println!("{item}");

    hybrid_tagged_impl(attr.into(), item.into())
}

fn hybrid_tagged_impl(attr: TokenStream2, item: TokenStream2) -> TokenStream {
    println!("{attr}");
    println!("{item}");

    let decl = venial::parse_declaration(item);

    let Ok(Declaration::Struct(typed_data)) = decl else {
        panic!("hybrid_tagged is meant to be invoked on a struct")
    };

    todo!()
}

#[cfg(test)]
mod test {
    use crate::hybrid_tagged_impl;
    use quote::quote;

    #[test]
    fn test_hybrid_tagged_impl() {
        hybrid_tagged_impl(quote!(
            tag = "type", variant = "varying"
        ), quote!(
            struct S {
                frame: Number,
                slack: Swick,
                varying: Variations,
            }
        ));
    }
}
