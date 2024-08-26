use bitflags::bitflags;

use crate::nds::Bits;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct DISPCNT(u32);

impl From<u32> for DISPCNT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl DISPCNT {
    const VRAM_BLOCK_OFFSET: u32 = 18;
    const VRAM_BLOCK_END: u32 = 19;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn get_vram_block(&self) -> u32 {
        self.0
            .get_bits(Self::VRAM_BLOCK_OFFSET, Self::VRAM_BLOCK_END)
    }
}

bitflags! {
    pub struct DisplayMode: u32 {
        const DISPLAY_OFF         = 0; // screen becomes white
        const GRAPHICS_DISPLAY    = 1; // normal BG and OBJ layers
        const VRAM_DISPLAY        = 2; // Engine A only: Bitmap from block selected in DISPCNT.18-19
        const MAIN_MEMORY_DISPLAY = 3; // Engine A only: Bitmap DMA transfer from Main RAM
    }
}
