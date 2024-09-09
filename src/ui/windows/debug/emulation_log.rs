use crate::{
    nds::logger::{
        self, has_error_to_show, set_has_error_to_show, set_pause_on_error, set_pause_on_warn,
    },
    ui::{NitrousUI, NitrousWindow},
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EmulationLogWindow {
    pub open: bool,

    pub pause_on_warn: bool,
    pub pause_on_error: bool,
    pub show_on_error: bool,
}

impl Default for EmulationLogWindow {
    fn default() -> Self {
        Self {
            open: false,
            pause_on_warn: false,
            pause_on_error: true,
            show_on_error: true,
        }
    }
}

impl EmulationLogWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.show_on_error && has_error_to_show() {
            self.open = true;
        }

        egui::Window::new_nitrous("Emulation Log", ctx)
            .default_width(600.0)
            .open(&mut self.open)
            .show(ctx, |ui| {
                set_has_error_to_show(false);

                egui::TopBottomPanel::top("emulation_log_navbar").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            logger::LOGS.lock().unwrap().clear();
                        }

                        let orig_pause_on_warn = self.pause_on_warn;
                        let mut pause_on_warn = orig_pause_on_warn;
                        ui.checkbox(&mut pause_on_warn, "Pause on Warn");
                        if pause_on_warn != orig_pause_on_warn {
                            set_pause_on_warn(pause_on_warn);
                            self.pause_on_warn = pause_on_warn;
                        }

                        let orig_pause_on_error = self.pause_on_error;
                        let mut pause_on_error = orig_pause_on_error;
                        ui.checkbox(&mut pause_on_error, "Pause on Error");
                        if pause_on_error != orig_pause_on_error {
                            set_pause_on_error(pause_on_error);
                            self.pause_on_error = pause_on_error;
                        }

                        ui.checkbox(&mut self.show_on_error, "Show on Error");
                    });

                    ui.add_space(4.0);
                });

                ui.add_space(4.0);

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
                                        "({}) {}",
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
