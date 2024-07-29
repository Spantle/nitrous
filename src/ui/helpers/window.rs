pub trait NitrousWindow {
    fn new_nitrous(title: impl Into<egui::WidgetText>, ctx: &egui::Context) -> Self;
}

impl NitrousWindow for egui::Window<'_> {
    fn new_nitrous(title: impl Into<egui::WidgetText>, _ctx: &egui::Context) -> Self {
        Self::new(title)
    }
}
