use self::models::{DISPCNT, POWCNT1};

mod models;

#[derive(Default)]
pub struct Gpu2d {
    pub dispcnt: DISPCNT, // 0x04000000
    pub powcnt1: POWCNT1, // 0x04000304
}

impl Gpu2d {
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
