mod background;

use crate::nds::{
    gpus::gpu2d::{BackgroundResults, Gpu2d, GpuRenderResult},
    Bits,
};

const COLOUR_MULT: f32 = 255.0 / 31.0;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    // Display Mode: Graphics Display
    pub fn render_graphics(&self) -> GpuRenderResult {
        let bg_mode = self.dispcnt.get_bg_mode();
        match bg_mode {
            0 => {
                let mut ids: Vec<usize> = (0..=3).collect();
                ids.sort_by(|&a, &b| {
                    self.bgxcnt[b]
                        .get_priority()
                        .cmp(&self.bgxcnt[a].get_priority())
                        .then(b.cmp(&a))
                });

                let mut bg_pixels: BackgroundResults = vec![(vec![vec![]], false); 4];
                if self.dispcnt.get_screen_display_bg0() {
                    bg_pixels[0] = self.render_background::<0>();
                }
                if self.dispcnt.get_screen_display_bg1() {
                    bg_pixels[1] = self.render_background::<1>();
                }
                if self.dispcnt.get_screen_display_bg2() {
                    bg_pixels[2] = self.render_background::<2>();
                }
                if self.dispcnt.get_screen_display_bg3() {
                    bg_pixels[3] = self.render_background::<3>();
                }

                let mut backdrop_colour_bytes = [0; 2];
                backdrop_colour_bytes.copy_from_slice(&self.palette[0..2]);
                let backdrop_colour = u16::from_le_bytes(backdrop_colour_bytes);
                let mut pixels: Vec<u16> = vec![backdrop_colour; 256 * 192];
                for id in ids {
                    if !bg_pixels[id].1 {
                        continue;
                    }

                    let bg = &bg_pixels[id].0;
                    let bg_width = bg.len();
                    let bg_height = bg[0].len();

                    let bg_x_offset = self.bghofs[id] as usize;
                    let bg_y_offset = self.bgvofs[id] as usize;

                    (0..256).for_each(|x| {
                        (0..192).for_each(|y| {
                            let i = y * 256 + x;
                            let x = (x + bg_x_offset) % bg_width;
                            let y = (y + bg_y_offset) % bg_height;

                            // leo taught me this fast conditional strat like a year ago
                            let new_pixel = bg[x][y];
                            let existing_pixel = pixels[i];

                            let is_transparent = !new_pixel.get_bit(15); // transparent: 0, normal: 1
                            let is_transparent_mask = (is_transparent as u16).wrapping_sub(1);
                            pixels[i] = (!is_transparent_mask & existing_pixel)
                                | (is_transparent_mask & new_pixel);
                        });
                    });
                }

                let image_data = egui::ImageData::from(egui::ColorImage {
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
                });

                GpuRenderResult::new(image_data, bg_pixels)
            }
            _ => GpuRenderResult::new_empty(egui::ImageData::from(egui::ColorImage {
                pixels: vec![egui::Color32::from_rgb(0, 0, 100 + 10 * bg_mode); 256 * 192],
                size: [256, 192],
            })),
        }
    }
}
