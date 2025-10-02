use crate::model_options::Options;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Result};

pub fn generate(attr: TokenStream, ast: ItemEnum) -> Result<TokenStream> {
    let options = syn::parse2::<Options>(attr)?;

    let mut tokens: Vec<TokenStream> = vec![];

    if options.python.is_some() {
        tokens.push(quote! {
            #[::pyo3::pyclass(eq, eq_int, frozen, hash, ord, str)]
        });
    }

    if let Some(ident) = options.abs {
        return Err(syn::Error::new_spanned(ident, "不支持抽象枚举类型"));
    } else if let Some((ident, _)) = options.ext {
        return Err(syn::Error::new_spanned(ident, "不支持扩展枚举类型"));
    }

    let name = &ast.ident;

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

        #[::pyo3::pymethods]
        impl #name {
            #[staticmethod]
            fn members() -> Vec<#name> {
                use strum::IntoEnumIterator;
                Self::iter().collect::<Vec<_>>()
            }
        }
    });

    Ok(quote! {
        #(#tokens)*
    })
}
