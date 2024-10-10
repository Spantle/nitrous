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

                let mut bg_pixels: Vec<(Vec<u16>, u32, u32)> = vec![(vec![], 0, 0); 4];
                bg_pixels[0] = self.draw_background::<0>();
                bg_pixels[1] = self.draw_background::<1>();
                bg_pixels[2] = self.draw_background::<2>();
                bg_pixels[3] = self.draw_background::<3>();

                let mut backdrop_colour_bytes = [0; 2];
                backdrop_colour_bytes.copy_from_slice(&self.palette[0..2]);
                let backdrop_colour = u16::from_le_bytes(backdrop_colour_bytes);
                let mut pixels: Vec<u16> = vec![backdrop_colour; 256 * 192];
                for id in ids {
                    let bg = &bg_pixels[id].0;
                    let bg_width = bg_pixels[id].1 as usize;
                    let bg_height = bg_pixels[id].2 as usize;

                    // TODO: translate the bg to the correct position

                    (0..pixels.len()).for_each(|i| {
                        let x = i % 256;
                        let y = i / 256;

                        // leo taught me this fast conditional strat like a year ago
                        let new_pixel = bg[(y % bg_height) * bg_width + (x % bg_width)];
                        let existing_pixel = pixels[i];

                        let is_transparent = !new_pixel.get_bit(15); // transparent: 0, normal: 1
                        let is_transparent_mask = (is_transparent as u16).wrapping_sub(1);
                        pixels[i] = (!is_transparent_mask & existing_pixel)
                            | (is_transparent_mask & new_pixel);
                    });
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
                    size: [256, 192],
                })
            }
            _ => egui::ImageData::from(egui::ColorImage {
                pixels: vec![egui::Color32::from_rgb(0, 0, 100 + 10 * bg_mode); 256 * 192],
                size: [256, 192],
            }),
        }
    }
}
