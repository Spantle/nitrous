use models::{BGxCNT, DispCnt, DisplayMode};

use crate::nds::shared::Shared;

pub mod models;
pub mod rendering;

// TODO: this might need to be refactored to be more like Bus
//       where we have a trait and use an Enum for Kind rather than const ENGINE_A
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Gpu2d<const ENGINE_A: bool> {
    pub dispcnt: DispCnt,
    pub bgxcnt: [BGxCNT; 4],

    pub bgofs: [u32; 4],

    pub palette: Vec<u8>,
    pub oam: Vec<u8>,
}

impl<const ENGINE_A: bool> Default for Gpu2d<ENGINE_A> {
    fn default() -> Self {
        Self {
            dispcnt: DispCnt::default(),
            bgxcnt: core::array::from_fn(|_| BGxCNT::default()),

            bgofs: [0; 4],

            palette: vec![0; 1024],
            oam: vec![0; 1024],
        }
    }
}

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn new_fake() -> Self {
        Self {
            dispcnt: DispCnt::default(),
            bgxcnt: core::array::from_fn(|_| BGxCNT::default()),

            bgofs: [0; 4],

            palette: vec![0; 0],
            oam: vec![0; 0],
        }
    }
}

pub type BackgroundResult = (Vec<Vec<u16>>, bool);
pub type BackgroundResults = Vec<BackgroundResult>;

pub struct GpuRenderResult {
    pub image_data: egui::ImageData,
    pub bgs: Option<BackgroundResults>,
}

impl GpuRenderResult {
    pub fn new(image_data: egui::ImageData, bgs: BackgroundResults) -> Self {
        Self {
            image_data,
            bgs: Some(bgs),
        }
    }

    pub fn new_empty(image_data: egui::ImageData) -> Self {
        Self {
            image_data,
            bgs: None,
        }
    }
}

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn render(&self, shared: &Shared) -> GpuRenderResult {
        let display_mode = self.dispcnt.get_display_mode();
        match display_mode {
            DisplayMode::DISPLAY_OFF => {
                return GpuRenderResult::new_empty(egui::ImageData::from(egui::ColorImage {
                    pixels: vec![egui::Color32::WHITE; 256 * 192],
                    size: [256, 192],
                }));
            }
            DisplayMode::GRAPHICS_DISPLAY => {} // continue as normal
            DisplayMode::VRAM_DISPLAY => {
                return GpuRenderResult::new_empty(self.render_vram(shared));
            }
            DisplayMode::MAIN_MEMORY_DISPLAY => {
                return GpuRenderResult::new_empty(egui::ImageData::from(egui::ColorImage {
                    pixels: vec![egui::Color32::DARK_RED; 256 * 192],
                    size: [256, 192],
                }));
            }
            _ => unreachable!("if you see this then i'm wrong. this is very much reachable"),
        };

        self.render_graphics(&shared.gpus.vram_banks)
    }
}
