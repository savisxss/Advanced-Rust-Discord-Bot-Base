use log::{Level, LevelFilter, Metadata, Record};
use chrono::Local;
use colored::*;
use std::sync::Once;

pub struct Logger;

static INIT: Once = Once::new();

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let level = record.level().to_string();
            let colored_level = match record.level() {
                Level::Error => level.red(),
                Level::Warn => level.yellow(),
                Level::Info => level.green(),
                Level::Debug => level.blue(),
                Level::Trace => level.purple(),
            };
            println!("[{}] {} - {} - {}", now, colored_level, record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    INIT.call_once(|| {
        log::set_logger(&Logger).unwrap();
        log::set_max_level(LevelFilter::Info);
    });
}

pub fn set_log_level(level: LevelFilter) {
    log::set_max_level(level);
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        log::error!($($arg)+);
    }
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        log::warn!($($arg)+);
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        log::info!($($arg)+);
    }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        log::debug!($($arg)+);
    }
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        log::trace!($($arg)+);
    }
}