use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result, Token, Type, Visibility, parse::ParseStream};

mod kw {
    syn::custom_keyword!(is);
    syn::custom_keyword!(to);
}

pub fn generate(ast: ParseStream) -> Result<TokenStream> {
    let vis = if ast.peek(Token![pub]) {
        ast.parse::<Visibility>()?
    } else {
        Visibility::Inherited
    };
    let name = ast.parse::<Ident>()?;
    ast.parse::<kw::is>()?;
    let key = ast.parse::<Box<Type>>()?;
    ast.parse::<kw::to>()?;
    let value = ast.parse::<Box<Type>>()?;

    let keys_iter_name = Ident::new(&format!("{name}KeysIterable"), name.span());
    let values_iter_name = Ident::new(&format!("{name}ValuesIterable"), name.span());
    let items_iter_name = Ident::new(&format!("{name}ItemsIterable"), name.span());

    let mut tokens: Vec<TokenStream> = vec![];

    tokens.push(quote! {
        #[::pyo3::pyclass(frozen, mapping)]
        #[derive(Clone, Default)]
        #vis struct #name(crate::types::alias::SafeMap<#key, #value>);

        impl From<crate::types::alias::SafeMap<#key, #value>> for #name {
            #[inline]
            fn from(value: crate::types::alias::SafeMap<#key, #value>) -> Self {
                Self(value)
            }
        }

        impl From<crate::types::alias::Map<#key, #value>> for #name {
            #[inline]
            fn from(value: crate::types::alias::Map<#key, #value>) -> Self {
                Self(crate::types::alias::new_safe(value))
            }
        }

        impl #name {
            #[inline]
            pub fn maps(&self) -> ::parking_lot::MappedRwLockReadGuard<crate::types::alias::Map<#key, #value>> {
                ::parking_lot::RwLockReadGuard::map(self.0.read(), |s| s)
            }
            #[inline]
            pub fn maps_mut(&self) -> ::parking_lot::MappedRwLockWriteGuard<crate::types::alias::Map<#key, #value>> {
                ::parking_lot::RwLockWriteGuard::map(self.0.write(), |s|  s)
            }
        }

        #[::pyo3::pymethods]
        impl #name {
            fn __getitem__(&self, key: #key) -> Option<#value> {
                self.maps().get(&key).cloned()
            }

            fn __len__(&self) -> usize {
                self.maps().len()
            }

            fn __contains__(&self, key: #key) -> bool {
                self.maps().contains_key(&key)
            }

            fn __iter__(&self) -> #keys_iter_name {
                #keys_iter_name {
                    data: self.clone(),
                    index: ::std::sync::Arc::new(::std::sync::atomic::AtomicUsize::new(0)),
                }
            }

            fn keys(&self) -> #keys_iter_name {
                #keys_iter_name {
                    data: self.clone(),
                    index: ::std::sync::Arc::new(::std::sync::atomic::AtomicUsize::new(0)),
                }
            }

            fn values(&self) -> #values_iter_name {
                #values_iter_name {
                    data: self.clone(),
                    index: ::std::sync::Arc::new(::std::sync::atomic::AtomicUsize::new(0)),
                }
            }

            fn items(&self) -> #items_iter_name {
                #items_iter_name {
                    data: self.clone(),
                    index: ::std::sync::Arc::new(::std::sync::atomic::AtomicUsize::new(0)),
                }
            }
        }
    });

    tokens.push(quote! {
        #[::pyo3::pyclass(frozen)]
        #[derive(Clone, Default)]
        #vis struct #keys_iter_name {
            pub data: #name,
            pub index: ::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        }

        #[::pyo3::pymethods]
        impl #keys_iter_name {
            fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyRef<'_, Self> {
                slf
            }

            fn __next__(mut slf: ::pyo3::PyRef<'_, Self>) -> Option<#key> {
                let index = slf.index.fetch_add(1, ::std::sync::atomic::Ordering::Release);
                if index < slf.data.maps().len() {
                    slf.data.maps().get_index(index).map(|(k, _)| k.clone())
                } else {
                    None
                }
            }
        }
    });

    tokens.push(quote! {
        #[::pyo3::pyclass(frozen)]
        #[derive(Clone, Default)]
        #vis struct #values_iter_name {
            pub data: #name,
            pub index: ::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        }

        #[::pyo3::pymethods]
        impl #values_iter_name {
            fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyRef<'_, Self> {
                slf
            }

            fn __next__(mut slf: ::pyo3::PyRef<'_, Self>) -> Option<#value> {
                let index = slf.index.fetch_add(1, ::std::sync::atomic::Ordering::Release);
                if index < slf.data.maps().len() {
                    slf.data.maps().get_index(index).map(|(_, v)| v.clone())
                } else {
                    None
                }
            }
        }
    });

    tokens.push(quote! {
        #[::pyo3::pyclass(frozen)]
        #[derive(Clone, Default)]
        #vis struct #items_iter_name {
            pub data: #name,
            pub index: ::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        }

        #[::pyo3::pymethods]
        impl #items_iter_name {
            fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyRef<'_, Self> {
                slf
            }

            fn __next__(mut slf: ::pyo3::PyRef<'_, Self>) -> Option<(#key, #value)> {
                let index = slf.index.fetch_add(1, ::std::sync::atomic::Ordering::Release);
                if index < slf.data.maps().len() {
                    slf.data.maps().get_index(index).map(|(k, v)| (k.clone(), v.clone()))
                } else {
                    None
                }
            }
        }
    });

    Ok(quote! { #(#tokens)* })
}
