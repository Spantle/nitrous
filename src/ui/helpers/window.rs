pub trait NitrousWindow {
    fn new_nitrous(title: impl Into<egui::WidgetText>, ctx: &egui::Context) -> Self;
    fn make_resizable(self, ctx: &egui::Context) -> Self;
}

impl NitrousWindow for egui::Window<'_> {
    fn new_nitrous(title: impl Into<egui::WidgetText>, ctx: &egui::Context) -> Self {
        Self::new(title).make_resizable(ctx)
    }

    fn make_resizable(self, ctx: &egui::Context) -> Self {
        let size = ctx.available_rect().size();

        self.hscroll(true)
            .vscroll(true)
            .resizable(true)
            .default_size(size)
            .max_size(size)
    }
}
