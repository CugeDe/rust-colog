extern crate log;
extern crate colored;
extern crate env_logger;

#[allow(unused_imports)]
use env_logger::Builder;
use log::LevelFilter;
use std::env;

mod format;
mod color;

#[allow(dead_code)]
pub fn builder() -> Builder
{
    let mut builder = Builder::new();
    builder.format(format::format);
    builder.filter(None, LevelFilter::Info);
    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }
    builder
}

pub fn init()
{
    drop(builder().init())
}
