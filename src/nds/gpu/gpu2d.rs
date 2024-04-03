use self::models::{DISPCNT, DISPSTAT};

mod models;

#[derive(Default)]
pub struct Gpu2d {
    pub dispcnt: DISPCNT,   // 0x04000000
    pub dispstat: DISPSTAT, // 0x04000004

    x: u32,
    y: u32,
}

impl Gpu2d {
    pub fn clock(&mut self) {
        self.x = (self.x + 1) % (256 + 99);
        self.y = (self.y + (self.x == 0) as u32) % (192 + 71);

        self.dispstat.set_hblank_flag(self.x >= 256);
        self.dispstat.set_vblank_flag(self.y >= 192);
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
