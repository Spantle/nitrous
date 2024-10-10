use crate::nds::{gpus::gpu2d::Gpu2d, shared::Shared, Bits};

const COLOUR_MULT: f32 = 255.0 / 31.0;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    // Display Mode: VRAM Display
    pub fn render_vram(&self, shared: &Shared) -> egui::ImageData {
        let mut pixels = Vec::with_capacity(256 * 192);
        for y in 0..=191 {
            for x in 0..=255 {
                let vram_block = self.dispcnt.get_vram_block();
                let addr_offset = (vram_block * 0x20000) as usize;
                let addr = addr_offset + (y * 256 + x) as usize * 2;
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
