use models::{DispCnt, DisplayMode};

use crate::nds::shared::Shared;

pub mod models;
mod rendering;

#[derive(Default)]
pub struct Gpu2d {
    pub dispcnt: DispCnt,
}

impl Gpu2d {
    pub fn render(&self, shared: &Shared) -> egui::ImageData {
        let display_mode = self.dispcnt.get_display_mode();
        match display_mode {
            DisplayMode::DISPLAY_OFF => {
                return egui::ImageData::from(egui::ColorImage {
                    pixels: vec![egui::Color32::WHITE; 256 * 192],
                    size: [256, 192],
                });
            }
            DisplayMode::GRAPHICS_DISPLAY => {} // continue as normal
            DisplayMode::VRAM_DISPLAY => {
                return self.render_vram(shared);
            }
            DisplayMode::MAIN_MEMORY_DISPLAY => {
                return egui::ImageData::from(egui::ColorImage {
                    pixels: vec![egui::Color32::DARK_RED; 256 * 192],
                    size: [256, 192],
                });
            }
            _ => unreachable!("if you see this then i'm wrong. this is very much reachable"),
        };

        egui::ImageData::from(egui::ColorImage {
            pixels: vec![egui::Color32::DARK_GREEN; 256 * 192],
            size: [256, 192],
        })
    }
}
