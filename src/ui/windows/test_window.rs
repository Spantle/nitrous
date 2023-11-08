use crate::ui::{NitrousGUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_test_window(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("Test Window", ctx)
            .open(&mut self.test_window)
            .show(ctx, |ui| {
                ui.heading("Heading");
            });
    }
}
