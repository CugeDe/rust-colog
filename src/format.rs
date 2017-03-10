use log::LogRecord;
use log::LogLevel;
use color;
use colored::*;

fn level_token(level: &LogLevel) -> &str
{
    match *level
    {
        LogLevel::Error => "ERROR",
        LogLevel::Warn  => "WARNING",
        LogLevel::Info  => "INFO",
        LogLevel::Debug => "DEBUG",
        LogLevel::Trace => "TRACE",
    }
}

fn prefix_token(level: &LogLevel) -> String
{
    format!("[{}]", color::level_color(level, level_token(level)))
}

pub fn format(record: &LogRecord) -> String
{
    let sep = format!("\n{} ", " | ".white().bold());
    format!(
        "{} {}",
        prefix_token(&record.level()),
        format!("{}", record.args()).replace("\n", &sep),
    )
}
