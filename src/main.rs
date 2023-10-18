extern crate env_logger;
#[macro_use]
extern crate log;

mod arm;
mod ui;

fn main() {
    let is_debug = cfg!(debug_assertions);
    let mut logger = env_logger::Builder::new();
    logger.filter_level(if is_debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });
    logger.parse_default_env();
    logger.init();

    info!("Initializing UI");
    let ui_result = ui::init();
    if ui_result.is_err() {
        error!("Error initializing UI: {}", ui_result.err().unwrap());
    }
}
