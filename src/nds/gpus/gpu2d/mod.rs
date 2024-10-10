use models::{BGxCNT, DispCnt, DisplayMode};

use crate::nds::shared::Shared;

pub mod models;
mod rendering;

// TODO: this might need to be refactored to be more like Bus
//       where we have a trait and use an Enum for Kind rather than const ENGINE_A

pub struct Gpu2d<const ENGINE_A: bool> {
    pub dispcnt: DispCnt,
    pub bgxcnt: [BGxCNT; 4],

    pub bg_vram: Vec<u8>,
    pub palette: Vec<u8>,
}

impl<const ENGINE_A: bool> Default for Gpu2d<ENGINE_A> {
    fn default() -> Self {
        let bg_vram_size = if ENGINE_A { 512 * 1024 } else { 128 * 1024 };

        Self {
            dispcnt: DispCnt::default(),
            bgxcnt: core::array::from_fn(|_| BGxCNT::default()),

            bg_vram: vec![0; bg_vram_size],
            palette: vec![0; 1024],
        }
    }
}

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
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

        self.render_graphics()
    }
}
