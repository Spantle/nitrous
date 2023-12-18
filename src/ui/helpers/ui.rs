pub trait NitrousUI {
    fn enabled_button(
        &mut self,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
        callback: impl FnOnce(),
    ) -> bool;
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
}
