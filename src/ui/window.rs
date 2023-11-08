pub trait NitrousWindow {
    fn new_nitrous(title: impl Into<egui::WidgetText>, ctx: &egui::Context) -> Self;
    fn make_resizable(self, ctx: &egui::Context) -> Self;
}

impl NitrousWindow for egui::Window<'_> {
    fn new_nitrous(title: impl Into<egui::WidgetText>, ctx: &egui::Context) -> Self {
        Self::new(title).make_resizable(ctx)
    }

    fn make_resizable(self, ctx: &egui::Context) -> Self {
        self.hscroll(true)
            .vscroll(true)
            .resizable(true)
            .resize(|resize| {
                let size = ctx.available_rect().size();
                resize.default_size(size).max_size(size)
            })
    }
}
