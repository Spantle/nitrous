mod background;

use crate::nds::{gpus::gpu2d::Gpu2d, Bits};

const COLOUR_MULT: f32 = 255.0 / 31.0;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    // Display Mode: Graphics Display
    pub fn render_graphics(&self) -> egui::ImageData {
        let bg_mode = self.dispcnt.get_bg_mode();
        match bg_mode {
            0 => {
                let mut ids: Vec<usize> = (0..=3).collect();
                ids.sort_by(|&a, &b| {
                    self.bgxcnt[a]
                        .get_priority()
                        .cmp(&self.bgxcnt[b].get_priority())
                        .then(a.cmp(&b))
                });

                let mut bg_pixels: Vec<Vec<u16>> = vec![vec![]; 4];
                bg_pixels[0] = self.draw_background::<0>();
                bg_pixels[1] = self.draw_background::<1>();
                bg_pixels[2] = self.draw_background::<2>();
                bg_pixels[3] = self.draw_background::<3>();

                let mut pixels: Vec<u16> = vec![0; 256 * 256];
                for id in ids {
                    let bg = &bg_pixels[id];
                    let start = 0;
                    let end = bg.len();

                    for i in start..end {
                        // leo taught me this fast conditional strat like a year ago
                        let new_pixel = bg[i];
                        let existing_pixel = pixels[i];

                        let is_transparent = !new_pixel.get_bit(15); // transparent: 0, normal: 1
                        let is_transparent_mask = (is_transparent as u16).wrapping_sub(1);
                        pixels[i] = (!is_transparent_mask & existing_pixel)
                            | (is_transparent_mask & new_pixel);
                    }
                }

                egui::ImageData::from(egui::ColorImage {
                    pixels: pixels
                        .iter()
                        .map(|&pixel| {
                            let r = ((pixel.get_bits(0, 4) as f32) * COLOUR_MULT) as u8;
                            let g = ((pixel.get_bits(5, 9) as f32) * COLOUR_MULT) as u8;
                            let b = ((pixel.get_bits(10, 14) as f32) * COLOUR_MULT) as u8;
                            egui::Color32::from_rgb(r, g, b)
                        })
                        .collect(),
                    size: [256, 256], // TODO: this is not how things work
                })
            }
            _ => egui::ImageData::from(egui::ColorImage {
                pixels: vec![egui::Color32::from_rgb(0, 0, 100 + 10 * bg_mode); 256 * 192],
                size: [256, 192],
            }),
        }
    }
}
