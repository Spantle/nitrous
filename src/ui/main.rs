use crate::emulator::Emulator;

pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(272.0, 427.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Nitrous",
        options,
        Box::new(|_cc| Box::<NitrousGUI>::new(NitrousGUI::new(emulator))),
    )
}

pub struct NitrousGUI {
    pub emulator: Emulator,

    pub test_window: bool,
}

impl NitrousGUI {
    pub fn new(emulator: Emulator) -> NitrousGUI {
        NitrousGUI {
            emulator,

            test_window: false,
        }
    }
}

impl eframe::App for NitrousGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        self.show_test_window(ctx);
    }
}
