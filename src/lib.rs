mod api;
mod consts;
mod global;
mod helpers;
mod models;
mod types;

use std::fs::File;
use std::process::exit;

use anyhow::Result;
use ctor::ctor;
use log::LevelFilter;
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};

use crate::global::CONFIG;

#[ctor]
fn init() {
    if CONFIG.orbit.log.write {
        log_panics::init();

        if setup_logger().is_err() {
            exit(1);
        }
    }
}

#[inline]
fn setup_logger() -> Result<()> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Debug)
            .set_time_format_str("%F %T%.3f")
            .build(),
        File::create(&CONFIG.orbit.log.path)?,
    )])?;

    Ok(())
}
