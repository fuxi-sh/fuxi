use serde::Serialize;
use serde_json::Value;
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use tinytemplate::TinyTemplate;

static TEMPLATE: &str = r###"from enum import Enum, auto
from typing import Optional
from .base import Market
from decimal import Decimal

class CoinCode(Enum):{{ for coin in coins }}
    { coin.variant } = auto(){{ endfor }}

    def id(self) -> str: ...
    def decimals(self) -> Decimal: ...

class SymbolCode(Enum):{{ for symbol in symbols }}
    { symbol.variant } = auto(){{ endfor }}

    def id(self) -> str: ...
    def market(self) -> Market: ...
    def is_spot(self) -> bool: ...
    def is_swap(self) -> bool: ...
    def base(self) -> Optional[CoinCode]: ...
    def quote(self) -> CoinCode: ...
    def decimals(self) -> Decimal: ...
    def code(self) -> str: ...
    def max_lever(self) -> Decimal: ...
"###;

#[derive(Serialize)]
struct HyperliquidCoin {
    pub id: String,
    pub variant: String,
    pub constants: String,
}

#[derive(Serialize)]
struct HyperliquidSymbol {
    pub variant: String,
    pub constants: String,
}

#[derive(Serialize)]
struct Context {
    pub coins: Vec<HyperliquidCoin>,
    pub symbols: Vec<HyperliquidSymbol>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = {
        let mut dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        dir.pop();
        dir.join("target").join("fuxi").join("hyerliquid")
    };
    std::fs::create_dir_all(&cache_dir)?;

    let spot_json_path = cache_dir.join("spot.json");
    let swap_json_path = cache_dir.join("swap.json");

    println!("cargo:rerun-if-changed={}", spot_json_path.display());
    println!("cargo:rerun-if-changed={}", swap_json_path.display());

    let mut context = Context {
        coins: vec![],
        symbols: vec![],
    };

    let spot_json: Value = serde_json::from_str(&std::fs::read_to_string(&spot_json_path)?)?;

    for token in spot_json[0]["tokens"].as_array().unwrap().iter() {
        let name = token["name"].as_str().unwrap().to_string();
        let variant = if matches!(name.as_bytes().first(), Some(b'0'..=b'9')) {
            format!("_{name}")
        } else {
            name.clone()
        }
        .to_uppercase();
        let constants = format!("CC_{}", name.to_uppercase());
        context.coins.push(HyperliquidCoin {
            id: name,
            variant,
            constants,
        });
    }

    for pair in spot_json[0]["universe"].as_array().unwrap().iter() {
        let tokens = pair["tokens"].as_array().unwrap();
        let base_index = tokens[0].as_u64().unwrap();
        let quote_index = tokens[1].as_u64().unwrap();
        if quote_index != 0 {
            continue;
        }
        let variant = context.coins[base_index as usize].variant.clone();
        let constants = format!(
            "SC_{}",
            context.coins[base_index as usize].id.to_uppercase()
        );
        context
            .symbols
            .push(HyperliquidSymbol { variant, constants });
    }

    let swap_json: Value = serde_json::from_str(&std::fs::read_to_string(&swap_json_path)?)?;

    for pair in swap_json[0]["universe"].as_array().unwrap().iter() {
        if pair["isDelisted"].as_bool().unwrap_or(false) {
            continue;
        }
        let name = pair["name"].as_str().unwrap().to_string();
        let variant = if matches!(name.as_bytes().first(), Some(b'0'..=b'9')) {
            format!("_{name}_SWAP")
        } else {
            format!("{name}_SWAP")
        }
        .to_uppercase();
        let constants = format!("SC_{}_SWAP", name.to_uppercase());
        context
            .symbols
            .push(HyperliquidSymbol { variant, constants });
    }

    let mut tt = TinyTemplate::new();
    tt.add_template("TEMPLATE", TEMPLATE)?;
    let python_code = tt.render("TEMPLATE", &context)?;

    let safe_path = Path::new("python/fuxi/_sdk/code.pyi");
    std::fs::create_dir_all(safe_path.parent().unwrap())?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(safe_path)?;
    f.write_all(python_code.as_bytes())?;

    Ok(())
}
