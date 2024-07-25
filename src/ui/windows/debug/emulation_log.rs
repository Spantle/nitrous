use crate::{
    nds::logger::{self, do_pause_on_warn, set_pause_on_warn},
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

                        let orig_pause_on_warn = do_pause_on_warn();
                        let mut pause_on_warn = orig_pause_on_warn;
                        ui.checkbox(&mut pause_on_warn, "Pause on Warn");
                        if pause_on_warn != orig_pause_on_warn {
                            set_pause_on_warn(pause_on_warn);
                        }
                    });
                });

                let log = logger::LOGS.lock().unwrap();
                let text_style = egui::TextStyle::Monospace;
                let height = ui.text_style_height(&text_style);
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show_rows(ui, height, log.len(), |ui, row_range| {
                        ui.make_monospace();

                        for row in row_range {
                            let line = &log[row];
                            ui.horizontal(|ui| {
                                // it's actually vertically centered but egui has these the wrong way around??????????
                                ui.horizontal_centered(|ui| {
                                    let (color, icon) = match line.kind {
                                        logger::LogKind::Debug => {
                                            let color = egui::Color32::LIGHT_BLUE;
                                            (color, ui.icon(color, egui_phosphor::regular::BUG))
                                        }
                                        logger::LogKind::Info => {
                                            let color = egui::Color32::LIGHT_GREEN;
                                            (color, ui.icon(color, egui_phosphor::regular::INFO))
                                        }
                                        logger::LogKind::Warn => {
                                            let color = egui::Color32::YELLOW;
                                            (color, ui.icon(color, egui_phosphor::regular::WARNING))
                                        }
                                        logger::LogKind::Error => {
                                            let color = egui::Color32::LIGHT_RED;
                                            (
                                                color,
                                                ui.icon(color, egui_phosphor::regular::X_CIRCLE),
                                            )
                                        }
                                    };

                                    icon.on_hover_text(format!(
                                        "({:?}) {}",
                                        &line.kind, &line.timestamp
                                    ));

                                    ui.colored_label(color, format!("[{}]", &line.source));

                                    ui.colored_label(color, &line.content);
                                });
                            });

                            ui.separator();
                        }
                    })
            });
    }
}

trait EmulationLogUi {
    fn icon(&mut self, color: egui::Color32, icon: &str) -> egui::Response;
}

impl EmulationLogUi for egui::Ui {
    fn icon(&mut self, color: egui::Color32, icon: &str) -> egui::Response {
        self.add(egui::widgets::Label::new(
            egui::RichText::new(icon)
                .color(color)
                .font(egui::FontId::default()),
        ))
    }
}
