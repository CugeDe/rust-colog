use color;
use colored::*;
use env_logger::fmt::Formatter;
use log::Record;
use log::Level;
use std::io::{Error, Write};

fn level_token(level: &Level) -> &str
{
    match *level
    {
        Level::Error => "ERROR",
        Level::Warn  => "WARNING",
        Level::Info  => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
}

fn prefix_token(level: &Level) -> String
{
    format!("[{}]", color::level_color(level, level_token(level)))
}

pub fn format(buf: &mut Formatter, record: &Record) -> Result<(), Error>
{
    let sep = format!("\n{} ", " | ".white().bold());
    writeln!(
        buf,
        "{} {}",
        prefix_token(&record.level()),
        format!("{}", record.args()).replace("\n", &sep),
    )
}
