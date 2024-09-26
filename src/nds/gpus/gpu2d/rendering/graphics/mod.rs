mod background;

use crate::nds::gpus::gpu2d::Gpu2d;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    // Display Mode: Graphics Display
    pub fn render_graphics(&self) -> egui::ImageData {
        let bg_mode = self.dispcnt.get_bg_mode();
        match bg_mode {
            0 => self.draw_background::<0>(),
            _ => egui::ImageData::from(egui::ColorImage {
                pixels: vec![egui::Color32::from_rgb(0, 0, 100 + 10 * bg_mode); 256 * 192],
                size: [256, 192],
            }),
        }
    }
}
