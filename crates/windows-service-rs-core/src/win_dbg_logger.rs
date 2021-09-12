use std::ffi::OsStr;
use winapi::um::debugapi;
use log::{LevelFilter, Metadata, Record};

pub struct DebuggerLogger;

impl log::Log for DebuggerLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            output_debug_string(format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) { }
}

fn output_debug_string(message: impl AsRef<OsStr>) {
    match widestring::WideCString::from_os_str(message) {
        Err(_) => return,
        Ok(message_string) => {
            unsafe { debugapi::OutputDebugStringW(message_string.as_ptr()); }
        }
    }
}

pub fn init() {
    log::set_boxed_logger(Box::new(DebuggerLogger))
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap_or_else(|e| output_debug_string(format!("Fail to initialize logger: {:?}", e)))
}