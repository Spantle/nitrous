use crate::ui::NitrousWindow;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TestWindow {
    pub open: bool,
}

impl TestWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("Test Window", ctx)
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.heading("Heading");
            });
    }
}
