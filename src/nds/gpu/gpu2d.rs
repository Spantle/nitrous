use crate::nds::{shared::Shared, Bits};

use super::models::{DISPCNT, DISPSTAT};

#[derive(Default)]
pub struct Gpu2d {
    pub dispcnt: DISPCNT,   // 0x04000000
    pub dispstat: DISPSTAT, // 0x04000004
    pub vcount: u16,        // 0x04000006

    x: u32,
}

const COLOUR_MULT: f32 = 255.0 / 31.0;

impl Gpu2d {
    pub fn clock(&mut self) {
        self.x = (self.x + 1) % (256 + 99);
        self.vcount = (self.vcount + (self.x == 0) as u16) % (192 + 71);

        self.dispstat.set_hblank_flag(self.x >= 256);
        self.dispstat
            .set_vblank_flag(self.vcount >= 192 && self.vcount != 262);

        let vcount_setting = self.dispstat.get_vcount_setting();
        self.dispstat.set_vcount_flag(vcount_setting == self.vcount);
    }

    pub fn render(&self, shared: &Shared) -> egui::ImageData {
        let mut pixels = Vec::with_capacity(256 * 192);
        for y in 0..=191 {
            for x in 0..=255 {
                let addr = (y * 256 + x) as usize * 2;
                let mut bytes = [0; 2];
                bytes.copy_from_slice(&shared.vram_lcdc_alloc[addr..addr + 2]);
                let halfword = u16::from_le_bytes(bytes);
                let r = ((halfword.get_bits(0, 4) as f32) * COLOUR_MULT) as u8;
                let g = ((halfword.get_bits(5, 9) as f32) * COLOUR_MULT) as u8;
                let b = ((halfword.get_bits(10, 14) as f32) * COLOUR_MULT) as u8;

                let pixel = egui::Color32::from_rgb(r, g, b);
                pixels.push(pixel);
            }
        }

        egui::ImageData::from(egui::ColorImage {
            pixels,
            size: [256, 192],
        })
    }
}
