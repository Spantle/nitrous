use std::{fmt::Display, sync::Mutex, time::SystemTime};

use once_cell::sync::Lazy;

pub static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug)]
pub struct Log {
    pub kind: LogKind,
    pub source: LogSource,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub enum LogKind {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub enum LogSource {
    Emu,
    Arm9,
    Bus9,
    Cart,
}

fn now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn debug<T: Into<String> + Display>(source: LogSource, content: T) {
    debug!("[{:?}] {}", source, &content);
    let mut logs = LOGS.lock().unwrap();
    logs.push(Log {
        kind: LogKind::Debug,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

pub fn info<T: Into<String> + Display>(source: LogSource, content: T) {
    info!("[{:?}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Info,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

pub fn warn<T: Into<String> + Display>(source: LogSource, content: T) {
    warn!("[{:?}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Warn,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

pub fn error<T: Into<String> + Display>(source: LogSource, content: T) {
    error!("[{:?}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Error,
        source,
        content: content.to_string(),
        timestamp: now(),
    })
}
