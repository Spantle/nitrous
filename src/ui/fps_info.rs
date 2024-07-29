use super::{FpsInfo, NitrousGUI};

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
                        ui.set_min_width(150.0);

                        ui.label(format!("FPS: {}", fps_info.estimated_fps));
                        ui.label(format!("Idle Time: {}ms", fps_info.idle_time / 1000));
                        ui.label(format!("UI Time: {}ms", fps_info.ui_time / 1000));
                        ui.label(format!(
                            "Compute Time: {}ms",
                            fps_info.estimated_compute_time / 1000
                        ));
                        ui.label(format!("Max Cycles: {}", fps_info.max_cycles));
                    });
            });
    }
}
