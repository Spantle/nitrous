use super::{NitrousGUI, NitrousWindow};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ScreenOptions {
    pub fit: bool,
    pub scale: f32,
    pub scale_text: String,
    pub horizontal_alignment: egui::Align,

    pub top_screens: Vec<u32>,
    pub top_screen_count: u32,
    pub bot_screens: Vec<u32>,
    pub bot_screen_count: u32,
    pub duo_screens: Vec<u32>,
    pub duo_screen_count: u32,
}

impl Default for ScreenOptions {
    fn default() -> Self {
        ScreenOptions {
            fit: true,
            scale: 1.0,
            scale_text: "1".to_string(),
            horizontal_alignment: egui::Align::Center,

            top_screens: Vec::new(),
            top_screen_count: 0,
            bot_screens: Vec::new(),
            bot_screen_count: 0,
            duo_screens: Vec::new(),
            duo_screen_count: 0,
        }
    }
}

impl NitrousGUI {
    pub fn render_screens(&mut self, ctx: &egui::Context) {
        let top_image_data = self.emulator.shared.gpu2d_a.render(&self.emulator.shared);
        let top_texture =
            ctx.load_texture("top_screen", top_image_data, egui::TextureOptions::NEAREST);
        let mut top_screen =
            egui::Image::from_texture(egui::load::SizedTexture::from_handle(&top_texture))
                .shrink_to_fit();
        let bot_image_data = self.emulator.shared.gpu2d_b.render(&self.emulator.shared);
        let bot_texture =
            ctx.load_texture("bot_screen", bot_image_data, egui::TextureOptions::NEAREST);
        let mut bot_screen =
            egui::Image::from_texture(egui::load::SizedTexture::from_handle(&bot_texture))
                .shrink_to_fit();

        self.display_windowed_screen(ctx, &top_screen, &bot_screen);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(self.screen_options.horizontal_alignment),
                |ui| {
                    let max_size = ui.available_size();
                    if self.screen_options.fit {
                        top_screen = top_screen.max_size(max_size);
                        bot_screen = bot_screen.max_size(max_size);
                    }

                    let size = if self.screen_options.fit {
                        egui_extras::Size::exact((max_size.y / 2.0).min(max_size.x / 256.0 * 192.0))
                    } else {
                        egui_extras::Size::exact(192.0 * self.screen_options.scale)
                    };

                    egui_extras::StripBuilder::new(ui)
                        .size(size)
                        .size(size)
                        .vertical(|mut strip| {
                            self.display_screen(&mut strip, top_screen);
                            self.display_screen(&mut strip, bot_screen);
                        });
                },
            );
        });
    }

    fn display_windowed_screen(
        &mut self,
        ctx: &egui::Context,
        top_screen: &egui::Image,
        bot_screen: &egui::Image,
    ) {
        for i in 0..self.screen_options.top_screens.len() {
            let id = self.screen_options.top_screens[i];
            let mut open = true;
            egui::Window::new_nitrous(format!("Top Screen #{}", i + 1), ctx)
                .id(egui::Id::new(format!("top_screen_{}", id)))
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_width(256.0 * self.screen_options.scale);
                    ui.set_height(192.0 * self.screen_options.scale);
                    ui.add(top_screen.clone());
                });

            if !open {
                self.screen_options.top_screens.retain(|&x| x != id);

                if self.screen_options.top_screens.is_empty() {
                    self.screen_options.top_screen_count = 0;
                }

                break; // windows will flicker, i don't care
            }
        }

        for i in 0..self.screen_options.bot_screens.len() {
            let id = self.screen_options.bot_screens[i];
            let mut open = true;
            egui::Window::new_nitrous(format!("Bot Screen #{}", i + 1), ctx)
                .id(egui::Id::new(format!("bot_screen_{}", id)))
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_width(256.0 * self.screen_options.scale);
                    ui.set_height(192.0 * self.screen_options.scale);
                    ui.add(bot_screen.clone());
                });

            if !open {
                self.screen_options.bot_screens.retain(|&x| x != id);

                if self.screen_options.bot_screens.is_empty() {
                    self.screen_options.bot_screen_count = 0;
                }

                break; // windows will flicker, i don't care
            }
        }

        for i in 0..self.screen_options.duo_screens.len() {
            let id = self.screen_options.duo_screens[i];
            let mut open = true;
            egui::Window::new_nitrous(format!("Dual Screen #{}", i + 1), ctx)
                .id(egui::Id::new(format!("duo_screen_{}", id)))
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_width(256.0 * self.screen_options.scale);
                    ui.set_height(192.0 * self.screen_options.scale * 2.0);
                    ui.add(top_screen.clone());
                    ui.add(bot_screen.clone());
                });

            if !open {
                self.screen_options.duo_screens.retain(|&x| x != id);

                if self.screen_options.duo_screens.is_empty() {
                    self.screen_options.duo_screen_count = 0;
                }

                break; // windows will flicker, i don't care
            }
        }
    }

    fn display_screen(&mut self, strip: &mut egui_extras::Strip<'_, '_>, image: egui::Image<'_>) {
        strip.strip(|builder| {
            builder
                .size(egui_extras::Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.add(image);
                    });
                });
        });
    }
}
