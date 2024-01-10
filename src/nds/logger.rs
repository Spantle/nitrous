use std::{fmt::Display, sync::Mutex};

use once_cell::sync::Lazy;

pub static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug)]
pub struct Log {
    pub kind: LogKind,
    pub content: String,
}

#[derive(Debug)]
pub enum LogKind {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn debug<T: Into<String> + Display>(content: T) {
    debug!("{}", &content);
    let mut logs = LOGS.lock().unwrap();
    logs.push(Log {
        kind: LogKind::Debug,
        content: content.to_string(),
    });
}

pub fn info<T: Into<String> + Display>(content: T) {
    info!("{}", &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Info,
        content: content.to_string(),
    });
}

pub fn warn<T: Into<String> + Display>(content: T) {
    warn!("{}", &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Warn,
        content: content.to_string(),
    });
}

pub fn error<T: Into<String> + Display>(content: T) {
    error!("{}", &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Error,
        content: content.to_string(),
    })
}
