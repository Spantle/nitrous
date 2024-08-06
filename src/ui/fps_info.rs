use super::{FpsInfo, NitrousGUI, NitrousUI};

impl NitrousGUI {
    pub fn show_fps_info(&mut self, ctx: &egui::Context, fps_info: FpsInfo) {
        if !self.fps_info {
            return;
        }

        egui::Area::new(egui::Id::new("fps_info"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 32.0))
            .show(ctx, |ui| {
                let window_fill = ui.style().visuals.window_fill();
                let fill = egui::Color32::from_rgba_unmultiplied(
                    window_fill.r(),
                    window_fill.g(),
                    window_fill.b(),
                    150,
                );

                egui::Frame::popup(ui.style())
                    .shadow(egui::Shadow::NONE)
                    .fill(fill)
                    .show(ui, |ui| {
                        ui.make_monospace();
                        ui.set_min_width(250.0);

                        let table = egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .column(egui_extras::Column::exact(150.0))
                            .column(egui_extras::Column::remainder());

                        table.body(|mut body| {
                            row(
                                &mut body,
                                "Measured FPS",
                                &format!("{}", fps_info.measured_fps),
                            );
                            row(
                                &mut body,
                                "Emulation Frame Time",
                                &format!("{}ms", fps_info.emulation_time),
                            );
                            row(
                                &mut body,
                                "UI Last Frame Time",
                                &format!("{}ms", fps_info.last_ui_time),
                            );
                            row(
                                &mut body,
                                "Idle Last Frame Time",
                                &format!("{}ms", fps_info.last_idle_time),
                            );
                            row(
                                &mut body,
                                "ARM9 Last Cycles Ran",
                                &format!("{}", fps_info.last_cycles_ran_arm9),
                            );
                            row(
                                &mut body,
                                "ARM9 Target Cycles",
                                &format!("{}", fps_info.target_cycles_arm9),
                            );
                            row(
                                &mut body,
                                "ARM9 Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_arm9),
                            );
                            row(
                                &mut body,
                                "ARM7 Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_arm7),
                            );
                            row(
                                &mut body,
                                "GPU Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_gpu),
                            );
                        });
                    });
            });
    }
}

fn row(body: &mut egui_extras::TableBody, label: &str, value: &str) {
    body.row(20.0, |mut row| {
        row.col(|ui| {
            ui.strong(label);
        });
        row.col(|ui| {
            ui.label(value);
        });
    });
}
