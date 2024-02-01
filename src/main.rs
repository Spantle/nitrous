use crate::nds::logger;

#[macro_use]
extern crate log;

mod nds;
mod ui;

fn main() {
    init();

    if cfg!(feature = "epic") {
        logger::info("Running in EPIC mode");
    } else {
        logger::info("Not running in epic mode");
    }

    let emulator = nds::Emulator::default();

    info!("Initializing UI");
    let ui_result = ui::init(emulator);
    if ui_result.is_err() {
        error!("Error initializing UI: {:?}", ui_result.err().unwrap());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn init() {
    let is_debug = cfg!(debug_assertions);
    let mut logger = env_logger::Builder::new();
    logger.filter_level(if is_debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });
    logger.parse_default_env();
    logger.init();
}

#[cfg(target_arch = "wasm32")]
fn init() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
}
