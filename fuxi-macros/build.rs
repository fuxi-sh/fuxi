use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Value, json};
use std::{fs::OpenOptions, io::Write, path::PathBuf, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = {
        let mut dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        dir.pop();
        dir.join("target").join("fuxi")
    };
    std::fs::create_dir_all(&dir)?;

    let spot_path = dir.join("spot.json");
    let swap_path = dir.join("swap.json");

    println!("cargo:rerun-if-changed={}", spot_path.display());
    println!("cargo:rerun-if-changed={}", swap_path.display());

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let http_client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    if !spot_path.exists() {
        let res = http_client
            .post("https://api.hyperliquid.xyz/info")
            .json(&json!({"type": "spotMetaAndAssetCtxs"}))
            .send()?
            .json::<Value>()?;
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&spot_path)?;
        f.write_all(serde_json::to_string_pretty(&res)?.as_bytes())?;
    }

    if !swap_path.exists() {
        let res = http_client
            .post("https://api.hyperliquid.xyz/info")
            .json(&json!({"type": "metaAndAssetCtxs"}))
            .send()?
            .json::<Value>()?;
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&swap_path)?;
        f.write_all(serde_json::to_string_pretty(&res)?.as_bytes())?;
    }

    Ok(())
}
