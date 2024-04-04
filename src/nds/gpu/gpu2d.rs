use super::models::{DISPCNT, DISPSTAT};

pub struct Gpu2d {
    pub dispcnt: DISPCNT,   // 0x04000000
    pub dispstat: DISPSTAT, // 0x04000004
    pub vcount: u16,        // 0x04000006

    x: u32,
}

impl Default for Gpu2d {
    fn default() -> Self {
        Self {
            dispcnt: DISPCNT::default(),
            dispstat: DISPSTAT::default(),
            // vcount: 191,
            vcount: 0,

            // x: 150,
            x: 0,
        }
    }
}

impl Gpu2d {
    pub fn clock(&mut self) {
        self.x = (self.x + 1) % (256 + 99);
        self.vcount = (self.vcount + (self.x == 0) as u16) % (192 + 71);

        self.dispstat.set_hblank_flag(self.x >= 256);
        self.dispstat.set_vblank_flag(self.vcount >= 192);
    }

    pub fn render(&self) -> egui::ImageData {
        let mut pixels = Vec::new();
        for y in 0..=191 {
            for x in 0..=255 {
                let pixel = egui::Color32::from_rgb(x, y, y);
                pixels.push(pixel);
            }
        }
        egui::ImageData::from(egui::ColorImage {
            pixels,
            size: [256, 192],
        })
    }
}
