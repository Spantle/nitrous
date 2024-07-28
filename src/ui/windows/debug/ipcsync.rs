use crate::{
    nds::shared::models::IpcsyncLog,
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_ipcsync_log(&mut self, ctx: &egui::Context) {
        self.emulator.shared.ipcsync.logging_enabled = self.ipcsync_log;

        egui::Window::new_nitrous("IPCSYNC Log", ctx)
            .default_width(100.0)
            .open(&mut self.ipcsync_log)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("ipcsync_log_navbar").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            self.emulator.shared.ipcsync.log = Vec::new();
                        }

                        let mut ipcsync_log_search = self.ipcsync_log_search.clone();
                        ui.add(
                            egui::TextEdit::singleline(&mut ipcsync_log_search).hint_text("Search"),
                        );
                        if self.ipcsync_log_search != ipcsync_log_search {
                            self.ipcsync_log_search = ipcsync_log_search;
                            if let Ok(result) = regex::Regex::new(&self.ipcsync_log_search) {
                                self.ipcsync_log_search_regex = result
                            }
                        }
                    });
                });

                let log = &self.emulator.shared.ipcsync.log;
                let filtered_log: Vec<&IpcsyncLog> = if self.ipcsync_log_search.is_empty() {
                    log.iter().collect()
                } else {
                    let mut filtered_log = vec![];
                    for line in log {
                        let (prefix, value) = match line {
                            IpcsyncLog::Read(is_arm9, value) => {
                                let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                (prefix, value)
                            }
                            IpcsyncLog::Write(is_arm9, value) => {
                                let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                (prefix, value)
                            }
                        };

                        if self
                            .ipcsync_log_search_regex
                            .is_match(&format!("[{}] {:08X}", prefix, value))
                        {
                            filtered_log.push(line);
                        }
                    }

                    filtered_log
                };

                let text_style = egui::TextStyle::Monospace;
                let height = ui.text_style_height(&text_style);
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show_rows(ui, height, filtered_log.len(), |ui, row_range| {
                        ui.make_monospace();

                        for row in row_range {
                            let line = &filtered_log[row];

                            let (color, icon, prefix, value) = match line {
                                IpcsyncLog::Read(is_arm9, value) => {
                                    let color = egui::Color32::LIGHT_GREEN;
                                    let icon = egui_phosphor::regular::ARROW_UP;
                                    let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                    (color, icon, prefix, value)
                                }
                                IpcsyncLog::Write(is_arm9, value) => {
                                    let color = egui::Color32::LIGHT_RED;
                                    let icon = egui_phosphor::regular::ARROW_DOWN;
                                    let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                    (color, icon, prefix, value)
                                }
                            };

                            ui.horizontal(|ui| {
                                // it's actually vertically centered but egui has these the wrong way around??????????
                                ui.horizontal_centered(|ui| {
                                    ui.icon(color, icon);
                                    ui.colored_label(color, format!("[{}]", &prefix));
                                    ui.colored_label(color, format!("{:08X}", value));
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
