use crate::nds::Emulator;

use super::NitrousGUI;

#[cfg(not(target_arch = "wasm32"))]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(egui::vec2(272.0, 427.0)),
        ..Default::default()
    };
    eframe::run_native(
        "NitrousDS",
        options,
        Box::new(|cc| Ok(creator(cc, emulator))),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("nitrous", options, Box::new(|cc| Ok(creator(cc, emulator))))
            .await
            .expect("Failed to run web app"); // TODO: can we pass this error out somehow?
    });

    Ok(())
}

fn creator(cc: &eframe::CreationContext, emulator: Emulator) -> Box<dyn eframe::App> {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    cc.egui_ctx.set_fonts(fonts);

    Box::new(NitrousGUI::new(cc, emulator))
}
