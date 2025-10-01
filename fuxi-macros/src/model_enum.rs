use crate::model_options::Options;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Result};

pub fn generate(attr: TokenStream, ast: ItemEnum) -> Result<TokenStream> {
    let options = syn::parse2::<Options>(attr)?;

    let mut tokens: Vec<TokenStream> = vec![];

    if options.python {
        tokens.push(quote! {
            #[::pyo3::pyclass(eq, eq_int, frozen, hash, ord, str)]
        });
    }

    tokens.push(quote! {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            ::std::hash::Hash,
            ::strum::Display,
            ::strum::EnumString,
            ::strum::AsRefStr,
            ::strum::EnumIter,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #ast
    });

    Ok(quote! {
        #(#tokens)*
    })
}
