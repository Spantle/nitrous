use super::{FpsInfo, NitrousGUI, NitrousUI};

impl NitrousGUI {
    pub fn show_fps_info(&mut self, ctx: &egui::Context, fps_info: FpsInfo) {
        if !self.fps_info {
            return;
        }

        egui::Area::new(egui::Id::new("fps_info"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 32.0))
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .shadow(egui::Shadow::NONE)
                    .show(ui, |ui| {
                        ui.make_monospace();
                        ui.set_min_width(250.0);

                        ui.label(format!("Estimated FPS: {}", fps_info.estimated_fps));
                        ui.label(format!(
                            "Last Frame Idle Time: {}ms",
                            fps_info.last_micros_idle / 1000
                        ));
                        ui.label(format!(
                            "Last Frame Emulation Time: {}ms",
                            fps_info.last_micros_emulation / 1000
                        ));
                        ui.label(format!(
                            "Last Frame UI Draw Time: {}ms",
                            fps_info.last_micros_ui / 1000
                        ));
                        ui.label(format!(
                            "Total Frame Time: {}ms",
                            fps_info.total_micros_frame / 1000
                        ));
                        ui.label(format!(
                            "Target Emulation Time: {}ms",
                            fps_info.target_emulation_time / 1000
                        ));
                        ui.label(format!(
                            "Target Cycles Per Frame: {}",
                            fps_info.target_cycles_per_frame
                        ));
                        ui.label(format!("Cycles Run: {}", fps_info.cycles_run));
                    });
            });
    }
}
