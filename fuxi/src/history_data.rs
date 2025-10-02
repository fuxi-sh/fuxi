use crate::{
    context::Context,
    types::base::{Codes, LogLevel, Market},
};
use anyhow::{Result, ensure};
use polars::prelude::*;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, time::Instant};

const DOWNLOAD_PREFIX: &str = "https://raw.githubusercontent.com/FrequentHippos/freqtrade_hyperliquid_download-data/refs/heads/main/user_data/data/hyperliquid/";

#[tokio::main(flavor = "current_thread")]
pub async fn download(context: Context, codes: &[Codes], force: bool) -> Result<()> {
    let dir = std::env::current_dir()?.join("data");
    let spot_dir = dir.join("spot");
    let swap_dir = dir.join("swap");
    std::fs::create_dir_all(&spot_dir)?;
    std::fs::create_dir_all(&swap_dir)?;

    let mut handles = vec![];
    for code in codes {
        let context = context.clone();
        let code = *code;

        let save_path = match code.market() {
            Market::Spot => spot_dir.join(format!(
                "{}.feather",
                code.code().replace("/", "_").replace(":", "_")
            )),
            Market::Swap => swap_dir.join(format!(
                "{}.feather",
                code.code().replace("/", "_").replace(":", "_")
            )),
        };
        if !force && save_path.exists() {
            continue;
        }

        let download_path = format!(
            "{DOWNLOAD_PREFIX}{}",
            match code.market() {
                Market::Spot => format!(
                    "{}-1m.feather",
                    code.code().replace("/", "_").replace(":", "_")
                ),
                Market::Swap => format!(
                    "futures/{}-1m-futures.feather",
                    code.code().replace("/", "_").replace(":", "_")
                ),
            }
        );

        let handle = tokio::spawn(async move {
            let start_time = Instant::now();

            context.engine_log(LogLevel::Debug, format_args!("{code}: k线下载中..."));

            let response = reqwest::get(download_path).await?;

            ensure!(
                response.status().is_success(),
                "下载失败: 交易对={code}, 状态={}",
                response.status()
            );

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(save_path.as_path())
                .await?;
            let bytes = response.bytes().await?;
            file.write_all(&bytes).await?;
            file.flush().await?;
            drop(file);

            tokio::task::spawn_blocking(move || {
                let mut df = LazyFrame::scan_ipc(
                    PlPathRef::from_local_path(save_path.as_path()).into_owned(),
                    Default::default(),
                )?;
                df = df.rename(["date"], ["time"], true);
                df = df.with_column(
                    col("time")
                        .dt()
                        .convert_time_zone(TimeZone::from_chrono(&chrono_tz::Asia::Shanghai)),
                );
                df = df.with_column(lit(true).alias("finished"));
                let mut df = df.collect()?;

                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(save_path.as_path())?;

                IpcWriter::new(&mut file).finish(&mut df)?;

                anyhow::Ok(())
            })
            .await??;

            let elapsed = start_time.elapsed();
            context.engine_log(
                LogLevel::Debug,
                format_args!(
                    "{code}: k线下载完成, 耗时={}",
                    humantime::format_duration(elapsed)
                ),
            );

            anyhow::Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
