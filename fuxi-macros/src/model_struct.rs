use crate::model_options::Options;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Ident, ItemStruct, Result, Visibility, spanned::Spanned};

pub fn generate(attr: TokenStream, ast: ItemStruct) -> Result<TokenStream> {
    let options = syn::parse2::<Options>(attr)?;
    let data_tokens = generate_data(&ast);
    let safe_tokens = generate_safe(&options, &ast);
    Ok(quote! {
        #data_tokens
        #safe_tokens
    })
}

fn generate_data(ast: &ItemStruct) -> TokenStream {
    let name = Ident::new(&format!("{}Data", ast.ident), ast.ident.span());
    let vis = &ast.vis;
    let attrs = &ast.attrs;

    let mut tokens: Vec<TokenStream> = vec![];

    tokens.push(quote! {
        #(#attrs)*
    });

    let mut field_tokens: Vec<TokenStream> = vec![];
    match &ast.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_vis = &field.vis;
                field_tokens.push(quote! {
                    #field_vis #field_name: #field_type,
                });
            }
        }
        Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter() {
                let field_type = &field.ty;
                let field_vis = &field.vis;
                field_tokens.push(quote! {
                    #field_vis #field_type,
                });
            }
        }
        Fields::Unit => {}
    }

    tokens.push(quote! {
       #vis struct #name {
        #(#field_tokens)*
       }
    });

    quote! { #(#tokens)* }
}

fn generate_safe(options: &Options, ast: &ItemStruct) -> TokenStream {
    let data_name = Ident::new(&format!("{}Data", ast.ident), ast.ident.span());
    let name = &ast.ident;
    let vis = &ast.vis;

    let mut tokens: Vec<TokenStream> = vec![];

    if options.python.is_some() {
        let mut support_tokens = vec![];
        support_tokens.push(quote! {
            frozen,
        });
        if options.abs.is_some() {
            support_tokens.push(quote! {
                subclass,
            });
        }
        if let Some((_, ext)) = &options.ext {
            support_tokens.push(quote! {
                extends=#ext,
            });
        }
        tokens.push(quote! {
            #[::pyo3::pyclass(#(#support_tokens)*)]
        });
    }

    tokens.push(quote! {
        #[derive(Clone)]
    });

    let mut field_get_set_tokens: Vec<TokenStream> = vec![];
    let mut py_field_get_tokens: Vec<TokenStream> = vec![];
    if let Fields::Named(fields) = &ast.fields {
        for (index, field) in fields.named.iter().enumerate() {
            let field_name = field.ident.as_ref();
            let field_type = &field.ty;
            let field_vis = &field.vis;

            let get_field_name = match field_name {
                Some(name) => name,
                None => &Ident::new(&format!("_{index}"), field.span()),
            };
            let get_mut_field_name = match field_name {
                Some(name) => &Ident::new(&format!("{name}_mut"), name.span()),
                None => &Ident::new(&format!("_{index}_mut"), field.span()),
            };
            field_get_set_tokens.push(quote! {
                #[inline]
                pub fn #get_field_name(&self) -> ::parking_lot::MappedRwLockReadGuard<#field_type> {
                    ::parking_lot::RwLockReadGuard::map(self.0.read(), |s| &s.#field_name)
                }
                #[inline]
                pub fn #get_mut_field_name(&self) -> ::parking_lot::MappedRwLockWriteGuard<#field_type> {
                    ::parking_lot::RwLockWriteGuard::map(self.0.write(), |s| &mut s.#field_name)
                }
            });

            let set_field_name = match field_name {
                Some(name) => &Ident::new(&format!("set_{name}"), name.span()),
                None => &Ident::new(&format!("set_{index}"), field.span()),
            };
            field_get_set_tokens.push(quote! {
                #[inline]
                pub fn #set_field_name(&self,value: #field_type) {
                    self.0.write().#field_name = value;
                }
            });

            if !matches!(field_vis, Visibility::Public(_)) {
                continue;
            }

            if options.python.is_some()
                && let Some(field_name) = field_name
            {
                let py_field_name = Ident::new(&format!("py_{field_name}"), field_name.span());
                py_field_get_tokens.push(quote! {
                    #[getter(#field_name)]
                    fn #py_field_name(&self) -> #field_type {
                        self.0.read().#field_name.clone()
                    }
                });
            }
        }
    }

    tokens.push(quote! {
        #vis struct #name(crate::types::alias::Safe<#data_name>);

        impl From<crate::types::alias::Safe<#data_name>> for #name {
            #[inline]
            fn from(value: crate::types::alias::Safe<#data_name>) -> Self {
                Self(value)
            }
        }

        impl From<#data_name> for #name {
            #[inline]
            fn from(value: #data_name) -> Self {
                Self(crate::types::alias::new_safe(value))
            }
        }

        impl #name {
            #(#field_get_set_tokens)*
        }
    });

    if options.python.is_some() {
        tokens.push(quote! {
            #[::pyo3::pymethods]
            impl #name {
                fn __repr__(&self) -> &str {
                    stringify!(#name)
                }

                #(#py_field_get_tokens)*
            }
        });
    }

    quote! { #(#tokens)* }
}
