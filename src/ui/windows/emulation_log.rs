use crate::{
    nds::logger,
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_emulation_log(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("Emulation Log", ctx)
            .default_width(600.0)
            .open(&mut self.emulation_log)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("emulation_log_navbar").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            logger::LOGS.lock().unwrap().clear();
                        }
                    });
                });

                let log = logger::LOGS.lock().unwrap();
                let text_style = egui::TextStyle::Monospace;
                let height = ui.text_style_height(&text_style);
                let total_rows = log.len() / 16;
                egui::ScrollArea::vertical().show_rows(ui, height, total_rows, |ui, row_range| {
                    ui.make_monospace();

                    for row in row_range {
                        let start = row * 16;
                        let end = start + 16;
                        let lines = &log[start..end];
                        for line in lines {
                            ui.horizontal(|ui| {
                                // it's actually vertically centered but egui has these the wrong way around??????????
                                ui.horizontal_centered(|ui| {
                                    let color = match line.kind {
                                        logger::LogKind::Debug => {
                                            let color = egui::Color32::LIGHT_BLUE;
                                            ui.icon(color, egui_phosphor::regular::BUG);
                                            color
                                        }
                                        logger::LogKind::Info => {
                                            let color = egui::Color32::LIGHT_GREEN;
                                            ui.icon(color, egui_phosphor::regular::INFO);
                                            color
                                        }
                                        logger::LogKind::Warn => {
                                            let color = egui::Color32::YELLOW;
                                            ui.icon(color, egui_phosphor::regular::WARNING);
                                            color
                                        }
                                        logger::LogKind::Error => {
                                            let color = egui::Color32::LIGHT_RED;
                                            ui.icon(color, egui_phosphor::regular::X_CIRCLE);
                                            color
                                        }
                                    };

                                    ui.colored_label(color, &line.content);
                                });
                            });

                            ui.separator();
                        }
                    }
                })
            });
    }
}

trait EmulationLogUi {
    fn icon(&mut self, color: egui::Color32, icon: &str);
}

impl EmulationLogUi for egui::Ui {
    fn icon(&mut self, color: egui::Color32, icon: &str) {
        self.add(egui::widgets::Label::new(
            egui::RichText::new(icon)
                .color(color)
                .font(egui::FontId::default()),
        ));
    }
}
