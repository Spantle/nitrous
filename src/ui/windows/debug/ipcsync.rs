use crate::{
    nds::{shared::models::IpcsyncLog, Emulator},
    ui::{NitrousUI, NitrousWindow},
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct IpcsyncLogWindow {
    pub open: bool,
}

impl IpcsyncLogWindow {
    pub fn show(&mut self, emulator: &mut Emulator, ctx: &egui::Context) {
        emulator.shared.ipcsync.logging_enabled = self.open;

        egui::Window::new_nitrous("IPCSYNC Log", ctx)
            .default_width(100.0)
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("ipcsync_log_navbar").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            emulator.shared.ipcsync.log = Vec::new();
                        }
                    });

                    ui.add_space(4.0);
                });

                ui.add_space(4.0);

                let log = &emulator.shared.ipcsync.log;
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
                                    let (color, prefix, value) = match line {
                                        IpcsyncLog::Read(is_arm9, value) => {
                                            let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                            let color = egui::Color32::LIGHT_GREEN;
                                            ui.icon(color, egui_phosphor::regular::ARROW_UP);
                                            (color, prefix, value)
                                        }
                                        IpcsyncLog::Write(is_arm9, value) => {
                                            let prefix = if *is_arm9 { "Bus9" } else { "Bus7" };
                                            let color = egui::Color32::LIGHT_RED;
                                            ui.icon(color, egui_phosphor::regular::ARROW_DOWN);
                                            (color, prefix, value)
                                        }
                                    };

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
