use anyhow::Result;
// use fast_log::plugin::{
//     file_split::{DateType, KeepType, Rolling, RollingType},
//     packer::LogPacker,
// };
use std::{fmt::Arguments, sync::Once};

static INIT: Once = Once::new();

pub fn init(chan_len: Option<usize>) {
    INIT.call_once(|| {
        // fastdate::set_offset_sec(8 * 60 * 60);
        // fast_log::init(
        //     fast_log::Config::new()
        //         .level(log::LevelFilter::Off)
        //         .chan_len(chan_len)
        //         .file_split(
        //             "logs/fuxi.log",
        //             Rolling::new(RollingType::ByDate(DateType::Day)),
        //             KeepType::KeepNum(2),
        //             LogPacker {},
        //         ),
        // )
        // .unwrap();

        fast_log::init(
            fast_log::Config::new()
                .console()
                .level(log::LevelFilter::Off)
                .chan_len(chan_len),
        )
        .unwrap();
    });
}

#[inline]
pub fn flush() -> Result<()> {
    fast_log::flush()?.wait();
    Ok(())
}

#[inline]
pub fn print(msg: Arguments) {
    let _ = fast_log::print(msg.to_string());
}
