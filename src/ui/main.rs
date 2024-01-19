use crate::nds::Emulator;

#[cfg(not(target_arch = "wasm32"))]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(egui::vec2(272.0, 427.0)),
        ..Default::default()
    };
    eframe::run_native("Nitrous", options, Box::new(|cc| creator(cc, emulator)))
}

#[cfg(target_arch = "wasm32")]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("nitrous", options, Box::new(|cc| creator(cc, emulator)))
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

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct NitrousGUI {
    #[serde(skip)]
    pub emulator: Emulator,

    pub arm9_info: bool,
    pub emulation_log: bool,
    pub memory_viewer: bool,
    pub test_window: bool,

    #[serde(skip)]
    pub memory_viewer_selected: Option<usize>,
    #[serde(skip)]
    pub memory_viewer_selected_pending_value: Option<u8>,
}

impl NitrousGUI {
    pub fn new(cc: &eframe::CreationContext<'_>, emulator: Emulator) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        NitrousGUI {
            emulator,

            ..Default::default()
        }
    }
}

impl eframe::App for NitrousGUI {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.emulator.clock();

        ctx.set_visuals(egui::Visuals {
            window_shadow: egui::epaint::Shadow {
                extrusion: 0.0,
                color: egui::Color32::TRANSPARENT,
            },
            ..Default::default()
        });

        self.show_navbar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::exact(192.0))
                .size(egui_extras::Size::exact(192.0))
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(egui_extras::Size::exact(256.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        0.0,
                                        egui::Color32::from_rgb(100, 0, 0),
                                    );
                                });
                            });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(egui_extras::Size::exact(256.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        0.0,
                                        egui::Color32::from_rgb(0, 0, 100),
                                    );
                                });
                            });
                    })
                });
        });

        self.show_arm9_info(ctx);
        self.show_emulation_log(ctx);
        self.show_memory_viewer(ctx);
        self.show_test_window(ctx);

        ctx.request_repaint();
    }
}
