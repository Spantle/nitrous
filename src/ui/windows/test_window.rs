use crate::ui::NitrousGUI;

impl NitrousGUI {
    pub fn show_test_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Test Window")
            .open(&mut self.test_window)
            .show(ctx, |ui| {
                ui.heading("Heading");
            });
    }
}
