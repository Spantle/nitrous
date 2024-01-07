use std::sync::Mutex;

use once_cell::sync::Lazy;

pub static LOGS: Lazy<Mutex<Vec<LogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug)]
pub struct LogEntry {
    pub kind: LogEntryKind,
    pub content: String,
}

#[derive(Debug)]
pub enum LogEntryKind {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn debug(content: String) {
    debug!("{}", &content);
    let mut logs = LOGS.lock().unwrap();
    logs.push(LogEntry {
        kind: LogEntryKind::Debug,
        content,
    });
}

pub fn info(content: String) {
    info!("{}", &content);
    LOGS.lock().unwrap().push(LogEntry {
        kind: LogEntryKind::Info,
        content,
    });
}

pub fn warn(content: String) {
    warn!("{}", &content);
    LOGS.lock().unwrap().push(LogEntry {
        kind: LogEntryKind::Warn,
        content,
    });
}

pub fn error(content: String) {
    error!("{}", &content);
    LOGS.lock().unwrap().push(LogEntry {
        kind: LogEntryKind::Error,
        content,
    })
}
