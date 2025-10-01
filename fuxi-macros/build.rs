use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Value, json};
use std::{fs::OpenOptions, io::Write, path::PathBuf, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = {
        let mut dir = PathBuf::from(std::env::var("OUT_DIR")?);
        dir.pop();
        dir.pop();
        dir.pop();
        dir.pop();
        dir
    };
    let cache_dir = target_dir.join("fuxi").join("hyerliquid");
    std::fs::create_dir_all(&cache_dir)?;

    let spot_json_path = cache_dir.join("spot.json");
    let swap_json_path = cache_dir.join("swap.json");

    println!("cargo:rerun-if-changed={}", spot_json_path.display());
    println!("cargo:rerun-if-changed={}", swap_json_path.display());

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let http_client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    if !spot_json_path.exists() {
        let res = http_client
            .post("https://api.hyperliquid.xyz/info")
            .json(&json!({"type": "spotMetaAndAssetCtxs"}))
            .send()?
            .json::<Value>()?;
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&spot_json_path)?;
        f.write_all(serde_json::to_string_pretty(&res)?.as_bytes())?;
    }

    if !swap_json_path.exists() {
        let res = http_client
            .post("https://api.hyperliquid.xyz/info")
            .json(&json!({"type": "metaAndAssetCtxs"}))
            .send()?
            .json::<Value>()?;
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&swap_json_path)?;
        f.write_all(serde_json::to_string_pretty(&res)?.as_bytes())?;
    }

    Ok(())
}
