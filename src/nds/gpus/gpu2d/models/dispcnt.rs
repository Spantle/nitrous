use bitflags::bitflags;

use crate::nds::Bits;

#[derive(Default)]
pub struct DispCnt(u32);

impl From<u32> for DispCnt {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl DispCnt {
    const BG_MODE_START: u32 = 0;
    const BG_MODE_END: u32 = 2;
    const BG0_2D_3D_SELECTION: u32 = 3;

    const SCREEN_DISPLAY_BG0: u32 = 8;
    const SCREEN_DISPLAY_BG1: u32 = 9;
    const SCREEN_DISPLAY_BG2: u32 = 10;
    const SCREEN_DISPLAY_BG3: u32 = 11;

    const DISPLAY_MODE_START: u32 = 16;
    const DISPLAY_MODE_END: u32 = 17;
    const VRAM_BLOCK_START: u32 = 18;
    const VRAM_BLOCK_END: u32 = 19;

    const CHARACTER_BASE_START: u32 = 24;
    const CHARACTER_BASE_END: u32 = 26;
    const SCREEN_BASE_START: u32 = 27;
    const SCREEN_BASE_END: u32 = 29;
    const BG_EXTENDED_PALETTES: u32 = 30;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn get_bg_mode(&self) -> u8 {
        self.0.get_bits(Self::BG_MODE_START, Self::BG_MODE_END) as u8
    }

    pub fn get_bg0_2d_3d_selection(&self) -> bool {
        self.0.get_bit(Self::BG0_2D_3D_SELECTION)
    }

    pub fn get_screen_display_bg0(&self) -> bool {
        self.0.get_bit(Self::SCREEN_DISPLAY_BG0)
    }

    pub fn get_screen_display_bg1(&self) -> bool {
        self.0.get_bit(Self::SCREEN_DISPLAY_BG1)
    }

    pub fn get_screen_display_bg2(&self) -> bool {
        self.0.get_bit(Self::SCREEN_DISPLAY_BG2)
    }

    pub fn get_screen_display_bg3(&self) -> bool {
        self.0.get_bit(Self::SCREEN_DISPLAY_BG3)
    }

    pub fn get_display_mode(&self) -> DisplayMode {
        DisplayMode::from_bits_truncate(
            self.0
                .get_bits(Self::DISPLAY_MODE_START, Self::DISPLAY_MODE_END),
        )
    }

    pub fn get_vram_block(&self) -> u32 {
        self.0
            .get_bits(Self::VRAM_BLOCK_START, Self::VRAM_BLOCK_END)
    }

    pub fn get_character_base(&self) -> u32 {
        self.0
            .get_bits(Self::CHARACTER_BASE_START, Self::CHARACTER_BASE_END)
    }

    pub fn get_screen_base(&self) -> u32 {
        self.0
            .get_bits(Self::SCREEN_BASE_START, Self::SCREEN_BASE_END)
    }

    pub fn get_bg_extended_palettes(&self) -> bool {
        self.0.get_bit(Self::BG_EXTENDED_PALETTES)
    }
}

bitflags! {
    #[derive(PartialEq)]
    pub struct DisplayMode: u32 {
        const DISPLAY_OFF         = 0; // screen becomes white
        const GRAPHICS_DISPLAY    = 1; // normal BG and OBJ layers
        const VRAM_DISPLAY        = 2; // Engine A only: Bitmap from block selected in DISPCNT.18-19
        const MAIN_MEMORY_DISPLAY = 3; // Engine A only: Bitmap DMA transfer from Main RAM
    }
}
