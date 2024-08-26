use crate::nds::{
    arm::bus::{bus7::Bus7, bus9::Bus9},
    shared::Shared,
    Bits,
};

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
    pub fn clock(&mut self, bus9: &mut Bus9, bus7: &mut Bus7) {
        self.x = (self.x + 1) % (256 + 99);
        self.vcount = (self.vcount + (self.x == 0) as u16) % (192 + 71);

        let hblanking = self.x >= 256;
        let hblank_start = self.x == 256;
        let vblanking = self.vcount >= 192 && self.vcount != 262;
        let vblank_start = self.vcount == 192;
        self.dispstat.set_hblank_flag(hblanking);
        self.dispstat.set_vblank_flag(vblanking);

        if hblank_start && self.dispstat.get_hblank_irq_enable() {
            bus9.interrupts.f.set_lcd_hblank(true);
            bus7.interrupts.f.set_lcd_hblank(true);
        }
        if vblank_start && self.dispstat.get_vblank_irq_enable() {
            bus9.interrupts.f.set_lcd_vblank(true);
            bus7.interrupts.f.set_lcd_vblank(true);
        }

        let vcount_setting = self.dispstat.get_vcount_setting();
        let is_vcounter_match = vcount_setting == self.vcount;
        self.dispstat.set_vcounter_flag(is_vcounter_match);
        if is_vcounter_match && self.dispstat.get_vcounter_irq_enable() {
            bus9.interrupts.f.set_lcd_vcounter_match(true);
            bus7.interrupts.f.set_lcd_vcounter_match(true);
        }
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
