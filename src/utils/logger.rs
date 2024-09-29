use log::{Level, LevelFilter, Metadata, Record};
use chrono::Local;
use colored::*;

pub struct Logger;

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
            println!("[{}] {} - {}", now, colored_level, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(LevelFilter::Info);
}