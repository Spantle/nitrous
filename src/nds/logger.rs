use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Mutex},
};

use once_cell::sync::Lazy;

use crate::nds::emulator::set_emulator_running;

pub static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(Vec::new()));
static PAUSE_ON_WARN: AtomicBool = AtomicBool::new(false);

pub trait LoggerTrait {
    fn log_debug<T: Into<String> + Display>(&self, content: T);
    fn log_info<T: Into<String> + Display>(&self, content: T);
    fn log_warn<T: Into<String> + Display>(&self, content: T);
    fn log_error<T: Into<String> + Display>(&self, content: T);
}

pub struct Logger(pub LogSource);

pub struct FakeLogger;

impl LoggerTrait for Logger {
    fn log_debug<T: Into<String> + Display>(&self, content: T) {
        debug(self.0, content);
    }

    fn log_info<T: Into<String> + Display>(&self, content: T) {
        info(self.0, content);
    }

    fn log_warn<T: Into<String> + Display>(&self, content: T) {
        warn(self.0, content);
    }

    fn log_error<T: Into<String> + Display>(&self, content: T) {
        error(self.0, content);
    }
}

impl LoggerTrait for FakeLogger {
    fn log_debug<T: Into<String> + Display>(&self, _content: T) {}
    fn log_info<T: Into<String> + Display>(&self, _content: T) {}
    fn log_warn<T: Into<String> + Display>(&self, _content: T) {}
    fn log_error<T: Into<String> + Display>(&self, _content: T) {}
}

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

#[derive(Debug, Clone, Copy)]
pub enum LogSource {
    Emu,
    Arm9(u32),
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
    if do_pause_on_warn() {
        set_emulator_running(false);
    }

    warn!("[{:?}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Warn,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

pub fn error<T: Into<String> + Display>(source: LogSource, content: T) {
    set_emulator_running(false);

    error!("[{:?}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Error,
        source,
        content: content.to_string(),
        timestamp: now(),
    })
}

pub fn do_pause_on_warn() -> bool {
    PAUSE_ON_WARN.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_pause_on_warn(pause: bool) {
    PAUSE_ON_WARN.store(pause, std::sync::atomic::Ordering::Relaxed);
}
