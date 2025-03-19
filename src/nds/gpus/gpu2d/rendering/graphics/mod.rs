mod background;
mod obj;

use crate::nds::{
    gpus::{
        gpu2d::{models::ColorSpecialEffect, BackgroundResults, Gpu2d, GpuRenderResult},
        vram::VramBanks,
    },
    Bits, IfElse,
};

const COLOUR_MULT: f32 = 255.0 / 31.0;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    // Display Mode: Graphics Display
    pub fn render_graphics(&self, vram_banks: &VramBanks) -> GpuRenderResult {
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

                let mut pixel_layers: BackgroundResults = vec![(vec![vec![]], false); 4];
                if self.dispcnt.get_screen_display_bg0()
                    && self.show_bgs[0]
                    && !self.dispcnt.get_bg0_2d_3d_selection()
                {
                    pixel_layers[0] = self.render_background::<0>(vram_banks);
                }
                if self.dispcnt.get_screen_display_bg1() && self.show_bgs[1] {
                    pixel_layers[1] = self.render_background::<1>(vram_banks);
                }
                if self.dispcnt.get_screen_display_bg2() && self.show_bgs[2] {
                    pixel_layers[2] = self.render_background::<2>(vram_banks);
                }
                if self.dispcnt.get_screen_display_bg3() && self.show_bgs[3] {
                    pixel_layers[3] = self.render_background::<3>(vram_banks);
                }
                let obj_layers = self.render_objs(vram_banks);

                let colorfx = self.bldcnt.get_color_special_effect();
                let eva = self.bldalpha[0].ev();
                let evb = self.bldalpha[1].ev();

                let mut backdrop_colour_bytes = [0; 2];
                backdrop_colour_bytes.copy_from_slice(&self.palette[0..2]);
                let backdrop_colour = u16::from_le_bytes(backdrop_colour_bytes);
                let mut pixels: Vec<u16> = vec![backdrop_colour; 256 * 192];
                for (i, id) in ids.iter().enumerate() {
                    let id: usize = *id;
                    if pixel_layers[id].1 {
                        let bg = &pixel_layers[id].0;
                        let bg_width = bg.len();
                        let bg_height = bg[0].len();

                        let (bg_x_offset, bg_y_offset) = (
                            self.bgofs[id].get_bits(0, 15) as usize,
                            self.bgofs[id].get_bits(16, 31) as usize,
                        );

                        let colorfx = match colorfx {
                            ColorSpecialEffect::AlphaBlending => {
                                if i != 0
                                    && self.bldcnt.get_first_target_pixel(ids[i] as u16)
                                    && self.bldcnt.get_second_target_pixel(ids[i - 1] as u16)
                                {
                                    ColorSpecialEffect::AlphaBlending
                                } else {
                                    ColorSpecialEffect::None
                                }
                            }
                            _ => ColorSpecialEffect::None,
                        };

                        (0..256).for_each(|x| {
                            (0..192).for_each(|y| {
                                let i = y * 256 + x;
                                let x = (x + bg_x_offset) % bg_width;
                                let y = (y + bg_y_offset) % bg_height;

                                let new_pixel = bg[x][y];
                                let existing_pixel = pixels[i];
                                match colorfx {
                                    ColorSpecialEffect::AlphaBlending => {
                                        pixels[i] = ColorSpecialEffect::alpha_blend(
                                            new_pixel,
                                            existing_pixel,
                                            eva,
                                            evb,
                                        );
                                    }
                                    _ => {
                                        let is_transparent = !new_pixel.get_bit(15); // transparent: 0, normal: 1
                                        pixels[i] =
                                            is_transparent.if_else(existing_pixel, new_pixel);
                                    }
                                }
                            });
                        });
                    }

                    if self.show_objs[3 - i] {
                        let obj_layer = &obj_layers[3 - i].0;
                        (0..256).for_each(|x| {
                            (0..192).for_each(|y| {
                                let i = y * 256 + x;

                                let new_pixel = obj_layer[x][y];
                                let existing_pixel = pixels[i];
                                let is_transparent = !new_pixel.get_bit(15); // transparent: 0, normal: 1
                                pixels[i] = is_transparent.if_else(existing_pixel, new_pixel);
                            });
                        });
                    }
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

                GpuRenderResult::new(image_data, pixel_layers, self.generate_tilemap(vram_banks))
            }
            _ => GpuRenderResult::new_empty(egui::ImageData::from(egui::ColorImage {
                pixels: vec![egui::Color32::from_rgb(0, 0, 100 + 10 * bg_mode); 256 * 192],
                size: [256, 192],
            })),
        }
    }
}
