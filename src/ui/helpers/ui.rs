pub trait NitrousUI {
    fn enabled_button(
        &mut self,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
        callback: impl FnOnce(),
    ) -> bool;
    fn make_monospace(&mut self);
    fn colored_strong(&mut self, color: egui::Color32, text: impl Into<String>) -> egui::Response;
}

impl NitrousUI for egui::Ui {
    fn enabled_button(
        &mut self,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
        callback: impl FnOnce(),
    ) -> bool {
        self.add_enabled_ui(enabled, |ui| {
            if ui.button(text).clicked() {
                callback();
                return true;
            }
            false
        })
        .inner
    }

    fn make_monospace(&mut self) {
        self.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
    }

    fn colored_strong(&mut self, color: egui::Color32, text: impl Into<String>) -> egui::Response {
        self.label(egui::RichText::new(text).color(color).strong())
    }
}
