use crate::{
    nds::{
        gpus::gpu2d::{BackgroundResult, BackgroundResults},
        Bits, IfElse,
    },
    ui::NitrousWindow,
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MapViewerWindow {
    pub open: bool,
}

// TODO: make this global or something stop copy pasting it everywhere
const COLOUR_MULT: f32 = 255.0 / 31.0;

// TODO: improve or finish this, i threw this together in a few minutes and don't care
impl MapViewerWindow {
    pub fn show(&mut self, ctx: &egui::Context, bgs: &Option<BackgroundResults>) {
        if !self.open {
            return;
        }

        if let Some(bgs) = bgs {
            let bgs = bgs
                .iter()
                .map(|bg| self.convert_pixels(bg))
                .collect::<Vec<_>>();

            egui::Window::new_nitrous("Map Viewer (WIP)", ctx)
                .open(&mut self.open)
                .show(ctx, |ui| {
                    for bg in bgs {
                        let tex = ctx.load_texture("top_screen", bg, egui::TextureOptions::NEAREST);

                        let img =
                            egui::Image::from_texture(egui::load::SizedTexture::from_handle(&tex));

                        ui.add(img);
                    }
                });
        }
    }

    fn convert_pixels(&self, bg: &BackgroundResult) -> egui::ImageData {
        let bg = &bg.0;
        let bg_width = bg.len();
        let bg_height = bg[0].len();
        let mut pixels: Vec<egui::Color32> = vec![egui::Color32::TRANSPARENT; bg_width * bg_height];

        let bg_x_offset = 0;
        let bg_y_offset = 0;

        (0..bg_width).for_each(|x| {
            (0..bg_height).for_each(|y| {
                let i = y * bg_width + x;
                let x = (x + bg_x_offset) % bg_width;
                let y = (y + bg_y_offset) % bg_height;

                let pixel = bg[x][y];
                let r = ((pixel.get_bits(0, 4) as f32) * COLOUR_MULT) as u8;
                let g = ((pixel.get_bits(5, 9) as f32) * COLOUR_MULT) as u8;
                let b = ((pixel.get_bits(10, 14) as f32) * COLOUR_MULT) as u8;

                pixels[i] = egui::Color32::from_rgba_premultiplied(
                    r,
                    g,
                    b,
                    pixel.get_bit(15).if_else(255, 0),
                );
            });
        });

        egui::ImageData::from(egui::ColorImage {
            pixels,
            size: [bg_width, bg_height],
        })
    }
}
