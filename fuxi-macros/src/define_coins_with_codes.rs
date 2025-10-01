use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_json::Value;
use std::path::PathBuf;
use syn::{Error, Ident, LitInt, LitStr, Result};

struct Coin {
    pub variant: Ident,
    pub id: LitStr,
    pub decimals: LitInt,
}

struct Symbol {
    pub variant: Ident,
    pub id: LitStr,
    pub decimals: LitInt,
    pub max_lever: LitInt,
    pub is_spot: bool,
    pub base_coin: Option<Ident>,
    pub code: LitStr,
}

pub fn generate() -> Result<TokenStream> {
    let mut tokens: Vec<TokenStream> = vec![];

    let dir = {
        let mut dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .map_err(|err| Error::new(Span::call_site(), err))?,
        );
        dir.pop();
        dir.join("target").join("fuxi")
    };

    let spot_path = dir.join("spot.json");
    let swap_path = dir.join("swap.json");

    let mut coins = vec![];
    let mut symbols = vec![];

    let spot_json: Value = serde_json::from_str(
        &std::fs::read_to_string(&spot_path).map_err(|err| Error::new(Span::call_site(), err))?,
    )
    .map_err(|err| Error::new(Span::call_site(), err))?;

    for token in spot_json[0]["tokens"]
        .as_array()
        .ok_or_else(|| Error::new(Span::call_site(), "代币不存在"))?
        .iter()
    {
        let name = token["name"].as_str().unwrap().to_string();
        let decimals = token["szDecimals"].as_u64().unwrap();
        let variant = Ident::new(
            if matches!(name.as_bytes().first(), Some(b'0'..=b'9')) {
                format!("_{name}")
            } else {
                name.clone()
            }
            .to_uppercase()
            .as_str(),
            Span::call_site(),
        );
        let id = LitStr::new(&name, Span::call_site());
        let decimals = LitInt::new(&decimals.to_string(), Span::call_site());
        coins.push(Coin {
            variant,
            id,
            decimals,
        });
    }

    for pair in spot_json[0]["universe"]
        .as_array()
        .ok_or_else(|| Error::new(Span::call_site(), "交易对不存在"))?
        .iter()
    {
        let name = pair["name"].as_str().unwrap().to_string();
        let tokens = pair["tokens"].as_array().unwrap();
        let base_index = tokens[0].as_u64().unwrap();
        let quote_index = tokens[1].as_u64().unwrap();
        if quote_index != 0 {
            continue;
        }
        let variant = coins[base_index as usize].variant.clone();
        let id = LitStr::new(&name, Span::call_site());
        let decimals = coins[base_index as usize].decimals.clone();
        let max_lever = LitInt::new("1", Span::call_site());
        let is_spot = true;
        let base_coin = Some(coins[base_index as usize].variant.clone());
        let code = LitStr::new(
            format!(
                "{}/USDC",
                coins[base_index as usize].id.value().to_uppercase()
            )
            .as_str(),
            Span::call_site(),
        );
        symbols.push(Symbol {
            variant,
            id,
            decimals,
            max_lever,
            is_spot,
            base_coin,
            code,
        });
    }

    let swap_json: Value = serde_json::from_str(
        &std::fs::read_to_string(&swap_path).map_err(|err| Error::new(Span::call_site(), err))?,
    )
    .map_err(|err| Error::new(Span::call_site(), err))?;

    for pair in swap_json[0]["universe"].as_array().unwrap().iter() {
        if let Some(flag) = pair["isDelisted"].as_bool()
            && flag
        {
            continue;
        }
        let name = pair["name"].as_str().unwrap().to_string();
        let max_lever = pair["maxLeverage"].as_u64().unwrap();
        let decimals = pair["szDecimals"].as_u64().unwrap();
        let variant = Ident::new(
            if matches!(name.as_bytes().first(), Some(b'0'..=b'9')) {
                format!("_{name}_SWAP")
            } else {
                format!("{name}_SWAP")
            }
            .to_uppercase()
            .as_str(),
            Span::call_site(),
        );
        let id = LitStr::new(&name, Span::call_site());
        let decimals = LitInt::new(&decimals.to_string(), Span::call_site());
        let max_lever = LitInt::new(&max_lever.to_string(), Span::call_site());
        let is_spot = false;
        let base_coin = None;
        let code = LitStr::new(
            format!("{}/USDC:USDC", name.to_uppercase()).as_str(),
            Span::call_site(),
        );
        symbols.push(Symbol {
            variant,
            id,
            decimals,
            max_lever,
            is_spot,
            base_coin,
            code,
        });
    }

    let mut coin_variant_tokens: Vec<TokenStream> = vec![];
    let mut coin_id_tokens: Vec<TokenStream> = vec![];
    let mut coin_decimals_tokens: Vec<TokenStream> = vec![];

    for coin in &coins {
        let variant = &coin.variant;
        let id = &coin.id;
        let decimals = &coin.decimals;
        coin_variant_tokens.push(quote! {
            #variant,
        });
        coin_id_tokens.push(quote! {
            Self::#variant => #id,
        });
        coin_decimals_tokens.push(quote! {
            Self::#variant => ::rust_decimal::dec!(#decimals),
        });
    }

    tokens.push(quote! {
        #[::fuxi_macros::model(python)]
        pub enum Coins {
            #(#coin_variant_tokens)*
        }

        #[::pyo3::pymethods]
        impl Coins {
            pub fn id(&self) -> &str {
                match self {
                    #(#coin_id_tokens)*
                }
            }
            pub fn decimals(&self) -> ::rust_decimal::Decimal {
                match self {
                    #(#coin_decimals_tokens)*
                }
            }
        }
    });

    let mut symbol_variant_tokens: Vec<TokenStream> = vec![];
    let mut symbol_id_tokens: Vec<TokenStream> = vec![];
    let mut symbol_market_tokens: Vec<TokenStream> = vec![];
    let mut symbol_is_spot_tokens: Vec<TokenStream> = vec![];
    let mut symbol_is_swap_tokens: Vec<TokenStream> = vec![];
    let mut symbol_base_tokens: Vec<TokenStream> = vec![];
    let mut symbol_decimals_tokens: Vec<TokenStream> = vec![];
    let mut symbol_code_tokens: Vec<TokenStream> = vec![];
    let mut symbol_max_lever_tokens: Vec<TokenStream> = vec![];

    for item in &symbols {
        let variant = &item.variant;
        let id = &item.id;
        let decimals = &item.decimals;
        let max_lever = &item.max_lever;
        let code = &item.code;
        symbol_variant_tokens.push(quote! {
            #variant,
        });
        symbol_id_tokens.push(quote! {
            Self::#variant => #id,
        });
        symbol_max_lever_tokens.push(quote! {
            Self::#variant => ::rust_decimal::dec!(#max_lever),
        });
        if item.is_spot {
            symbol_market_tokens.push(quote! {
                Self::#variant => Market::Spot,
            });
            symbol_is_spot_tokens.push(quote! {
                Self::#variant => true,
            });
            symbol_is_swap_tokens.push(quote! {
                Self::#variant => false,
            });
        } else {
            symbol_market_tokens.push(quote! {
                Self::#variant => Market::Swap,
            });
            symbol_is_spot_tokens.push(quote! {
                Self::#variant => false,
            });
            symbol_is_swap_tokens.push(quote! {
                Self::#variant => true,
            });
        }
        if let Some(base_coin) = &item.base_coin {
            symbol_base_tokens.push(quote! {
                Self::#variant => Some(Coins::#base_coin),
            });
        } else {
            symbol_base_tokens.push(quote! {
                Self::#variant => None,
            });
        }
        symbol_decimals_tokens.push(quote! {
            Self::#variant => ::rust_decimal::dec!(#decimals),
        });
        symbol_code_tokens.push(quote! {
            Self::#variant => #code,
        });
    }

    tokens.push(quote! {
        #[::fuxi_macros::model(python)]
        pub enum Codes {
            #(#symbol_variant_tokens)*
        }

        #[::pyo3::pymethods]
        impl Codes {
            pub fn id(&self) -> &str {
                match self {
                    #(#symbol_id_tokens)*
                }
            }
            pub fn market(&self) -> Market {
                match self {
                    #(#symbol_market_tokens)*
                }
            }
            pub fn is_spot(&self) -> bool {
                match self {
                    #(#symbol_is_spot_tokens)*
                }
            }
            pub fn is_swap(&self) -> bool {
                match self {
                    #(#symbol_is_swap_tokens)*
                }
            }
            pub fn base(&self) -> Option<Coins> {
                match self {
                    #(#symbol_base_tokens)*
                }
            }
            pub fn quote(&self) -> Coins {
                Coins::USDC
            }
            pub fn decimals(&self) -> ::rust_decimal::Decimal {
                match self {
                    #(#symbol_decimals_tokens)*
                }
            }
            pub fn code(&self) -> &str {
                match self {
                    #(#symbol_code_tokens)*
                }
            }
            pub fn max_lever(&self) -> ::rust_decimal::Decimal {
                match self {
                    #(#symbol_max_lever_tokens)*
                }
            }
        }
    });

    Ok(quote! { #(#tokens)* })
}
