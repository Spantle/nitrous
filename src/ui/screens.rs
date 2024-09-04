use super::NitrousGUI;

impl NitrousGUI {
    pub fn render_screens(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::exact(192.0))
                .size(egui_extras::Size::exact(192.0))
                .vertical(|mut strip| {
                    self.render_screen::<true>(&mut strip);
                    self.render_screen::<false>(&mut strip);
                });
        });
    }

    fn render_screen<const TOP_SCREEN: bool>(&mut self, strip: &mut egui_extras::Strip<'_, '_>) {
        let (gpu, texture_name) = if TOP_SCREEN {
            (&self.emulator.shared.gpu2d_a, "top_screen")
        } else {
            (&self.emulator.shared.gpu2d_b, "bottom_screen")
        };

        strip.strip(|builder| {
            builder
                .size(egui_extras::Size::exact(256.0))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        let image = gpu.render(&self.emulator.shared);
                        let texture =
                            ui.ctx()
                                .load_texture(texture_name, image, Default::default());
                        ui.add(egui::Image::from_texture(
                            egui::load::SizedTexture::from_handle(&texture),
                        ));
                    });
                });
        });
    }
}
