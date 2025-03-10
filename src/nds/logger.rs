#![allow(dead_code)]

use std::{
    collections::HashSet,
    fmt::Display,
    sync::{atomic::AtomicBool, Mutex},
};

use once_cell::sync::Lazy;

use crate::nds::emulator::set_emulator_running;

pub static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(Vec::new()));
static PAUSE_ON_WARN: AtomicBool = AtomicBool::new(false);
static PAUSE_ON_ERROR: AtomicBool = AtomicBool::new(false);
static HAS_ERROR_TO_SHOW: AtomicBool = AtomicBool::new(false);

pub static ONCE_LOGS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub trait LoggerTrait {
    fn set_source(&mut self, source: LogSource);

    fn log_debug<T: Into<String> + Display>(&self, content: T);
    fn log_info<T: Into<String> + Display>(&self, content: T);
    fn log_warn<T: Into<String> + Display>(&self, content: T);
    fn log_error<T: Into<String> + Display>(&self, content: T);

    fn log_warn_once<T: Into<String> + Display>(&self, content: T);
    fn log_error_once<T: Into<String> + Display>(&self, content: T);
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Logger(pub LogSource);

pub struct FakeLogger;

impl LoggerTrait for Logger {
    fn set_source(&mut self, source: LogSource) {
        self.0 = source;
    }

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

    fn log_warn_once<T: Into<String> + Display>(&self, content: T) {
        warn_once(self.0, content);
    }

    fn log_error_once<T: Into<String> + Display>(&self, content: T) {
        error_once(self.0, content);
    }
}

impl LoggerTrait for FakeLogger {
    fn set_source(&mut self, _source: LogSource) {}
    fn log_debug<T: Into<String> + Display>(&self, _content: T) {}
    fn log_info<T: Into<String> + Display>(&self, _content: T) {}
    fn log_warn<T: Into<String> + Display>(&self, _content: T) {}
    fn log_error<T: Into<String> + Display>(&self, _content: T) {}
    fn log_warn_once<T: Into<String> + Display>(&self, _content: T) {}
    fn log_error_once<T: Into<String> + Display>(&self, _content: T) {}
}

pub struct Log {
    pub kind: LogKind,
    pub source: LogSource,
    pub content: String,
    pub timestamp: String,
}

pub enum LogKind {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum LogSource {
    Emu,
    Arm9(u32),
    Arm7(u32),
    Arm9T(u16),
    Arm7T(u16),
    Bus7,
    Bus9,
    DMA9,
    DMA7,
    Cart,
    VramBank(u8),
    Spi,
}

impl Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogKind::Debug => write!(f, "Debug"),
            LogKind::Info => write!(f, "Info"),
            LogKind::Warn => write!(f, "Warn"),
            LogKind::Error => write!(f, "Error"),
        }
    }
}

impl Display for LogSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogSource::Emu => write!(f, "Emu"),
            LogSource::Arm9(instruction) => {
                if *instruction == 0 {
                    write!(f, "Arm9")
                } else {
                    write!(f, "Arm9({:08X})", instruction)
                }
            }
            LogSource::Arm7(instruction) => {
                if *instruction == 0 {
                    write!(f, "Arm7")
                } else {
                    write!(f, "Arm7({:08X})", instruction)
                }
            }
            LogSource::Arm9T(instruction) => {
                if *instruction == 0 {
                    write!(f, "Arm9T")
                } else {
                    write!(f, "Arm9T({:04X})", instruction)
                }
            }
            LogSource::Arm7T(instruction) => {
                if *instruction == 0 {
                    write!(f, "Arm7T")
                } else {
                    write!(f, "Arm7T({:04X})", instruction)
                }
            }
            LogSource::Bus7 => write!(f, "Bus7"),
            LogSource::Bus9 => write!(f, "Bus9"),
            LogSource::DMA9 => write!(f, "DMA9"),
            LogSource::DMA7 => write!(f, "DMA7"),
            LogSource::Cart => write!(f, "Cart"),
            LogSource::VramBank(id) => write!(f, "VramBank({})", id),
            LogSource::Spi => write!(f, "SPI"),
        }
    }
}

fn now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(debug_assertions)]
pub fn debug<T: Into<String> + Display>(source: LogSource, content: T) {
    debug!("[{}] {}", source, &content);
    let mut logs = LOGS.lock().unwrap();
    logs.push(Log {
        kind: LogKind::Debug,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

#[cfg(not(debug_assertions))]
pub fn debug<T>(_: LogSource, _: T) {}

pub fn debug_release<T: Into<String> + Display>(source: LogSource, content: T) {
    if cfg!(debug_assertions) {
        debug!("[{}] {}", source, &content);
    } else {
        info!("[{}] {}", source, &content);
    }
    let mut logs = LOGS.lock().unwrap();
    logs.push(Log {
        kind: LogKind::Debug,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

pub fn info<T: Into<String> + Display>(source: LogSource, content: T) {
    info!("[{}] {}", source, &content);
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

    warn!("[{}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Warn,
        source,
        content: content.to_string(),
        timestamp: now(),
    });
}

#[cfg(debug_assertions)]
pub fn warn_once<T: Into<String> + Display>(source: LogSource, content: T) {
    if !ONCE_LOGS.lock().unwrap().insert(content.to_string()) {
        return;
    }

    warn(source, content);
}

#[cfg(not(debug_assertions))]
pub fn warn_once<T>(_: LogSource, _: T) {}

pub fn error<T: Into<String> + Display>(source: LogSource, content: T) {
    if do_pause_on_error() {
        set_emulator_running(false);
    }

    set_has_error_to_show(true);

    error!("[{}] {}", source, &content);
    LOGS.lock().unwrap().push(Log {
        kind: LogKind::Error,
        source,
        content: content.to_string(),
        timestamp: now(),
    })
}

pub fn error_once<T: Into<String> + Display>(source: LogSource, content: T) {
    if !ONCE_LOGS.lock().unwrap().insert(content.to_string()) {
        return;
    }

    error(source, content);
}

pub fn do_pause_on_warn() -> bool {
    PAUSE_ON_WARN.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_pause_on_warn(pause: bool) {
    PAUSE_ON_WARN.store(pause, std::sync::atomic::Ordering::Relaxed);
}

pub fn do_pause_on_error() -> bool {
    PAUSE_ON_ERROR.load(std::sync::atomic::Ordering::Relaxed)
}
pub fn set_pause_on_error(pause: bool) {
    PAUSE_ON_ERROR.store(pause, std::sync::atomic::Ordering::Relaxed);
}

pub fn has_error_to_show() -> bool {
    HAS_ERROR_TO_SHOW.load(std::sync::atomic::Ordering::Relaxed)
}
pub fn set_has_error_to_show(show: bool) {
    HAS_ERROR_TO_SHOW.store(show, std::sync::atomic::Ordering::Relaxed);
}

macro_rules! format_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            format!($($arg)*)
        } else {
            String::new()
        }
    };
}
pub(crate) use format_debug;
