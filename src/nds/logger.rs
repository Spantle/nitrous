use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Mutex},
};

use once_cell::sync::Lazy;

use crate::nds::emulator::set_emulator_running;

pub static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(Vec::new()));
static PAUSE_ON_WARN: AtomicBool = AtomicBool::new(false);

pub trait LoggerTrait {
    fn set_source(&mut self, source: LogSource);

    fn log_debug<T: Into<String> + Display>(&self, content: T);
    fn log_info<T: Into<String> + Display>(&self, content: T);
    fn log_warn<T: Into<String> + Display>(&self, content: T);
    fn log_error<T: Into<String> + Display>(&self, content: T);
}

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
}

impl LoggerTrait for FakeLogger {
    fn set_source(&mut self, _source: LogSource) {}
    fn log_debug<T: Into<String> + Display>(&self, _content: T) {}
    fn log_info<T: Into<String> + Display>(&self, _content: T) {}
    fn log_warn<T: Into<String> + Display>(&self, _content: T) {}
    fn log_error<T: Into<String> + Display>(&self, _content: T) {}
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

#[derive(Clone, Copy)]
pub enum LogSource {
    Emu,
    Arm9(u32),
    Arm7(u32),
    Arm9T(u16),
    Arm7T(u16),
    Bus7,
    Bus9,
    Cart,
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
            LogSource::Cart => write!(f, "Cart"),
        }
    }
}

fn now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

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

pub fn error<T: Into<String> + Display>(source: LogSource, content: T) {
    set_emulator_running(false);

    error!("[{}] {}", source, &content);
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
