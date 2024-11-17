use crate::nds::gpus::gpu2d::GpuRenderResult;

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

    #[serde(skip)]
    pub engine_a_result: GpuRenderResult,
    #[serde(skip)]
    pub engine_b_result: GpuRenderResult,
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

            engine_a_result: GpuRenderResult::default(),
            engine_b_result: GpuRenderResult::default(),
        }
    }
}

impl NitrousGUI {
    pub fn render_screens(&mut self, ctx: &egui::Context) {
        if self.emulator.shared.gpus.vblank_flag {
            self.emulator.shared.gpus.vblank_flag = false;
            self.screen_options.engine_a_result =
                self.emulator.shared.gpus.a.render(&self.emulator.shared);
            self.screen_options.engine_b_result =
                self.emulator.shared.gpus.b.render(&self.emulator.shared);
        }

        let engine_a_result = self.screen_options.engine_a_result.clone();
        let engine_b_result = self.screen_options.engine_b_result.clone();

        let (top_texture, bot_texture) = if self.emulator.shared.powcnt1.get_display_swap() {
            (
                ctx.load_texture(
                    "top_screen",
                    engine_a_result.image_data,
                    egui::TextureOptions::NEAREST,
                ),
                ctx.load_texture(
                    "bot_screen",
                    engine_b_result.image_data,
                    egui::TextureOptions::NEAREST,
                ),
            )
        } else {
            (
                ctx.load_texture(
                    "top_screen",
                    engine_b_result.image_data,
                    egui::TextureOptions::NEAREST,
                ),
                ctx.load_texture(
                    "bot_screen",
                    engine_a_result.image_data,
                    egui::TextureOptions::NEAREST,
                ),
            )
        };

        let mut top_screen =
            egui::Image::from_texture(egui::load::SizedTexture::from_handle(&top_texture))
                .shrink_to_fit();
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

        self.gpu_map_viewer.show(ctx, &engine_b_result.bgs);
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
