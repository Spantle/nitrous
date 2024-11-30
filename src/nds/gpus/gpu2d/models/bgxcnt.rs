use crate::nds::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct BGxCNT(u16);

impl From<u16> for BGxCNT {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl BGxCNT {
    const PRIORITY_START: u16 = 0;
    const PRIORITY_END: u16 = 1;
    const CHARACTER_BASE_BLOCK_START: u16 = 2;
    const CHARACTER_BASE_BLOCK_END: u16 = 5;

    const COLOR_PALETTE_OFFSET: u16 = 7;
    const SCREEN_BASE_BLOCK_START: u16 = 8;
    const SCREEN_BASE_BLOCK_END: u16 = 12;
    const EXT_PALETTE_SLOT_OFFSET: u16 = 13;
    const SCREEN_SIZE_START: u16 = 14;
    const SCREEN_SIZE_END: u16 = 15;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn get_priority(&self) -> u8 {
        self.0.get_bits(Self::PRIORITY_START, Self::PRIORITY_END) as u8
    }

    pub fn get_character_base_block(&self) -> u16 {
        self.0.get_bits(
            Self::CHARACTER_BASE_BLOCK_START,
            Self::CHARACTER_BASE_BLOCK_END,
        )
    }

    pub fn get_color_palette(&self, extended_palettes: bool) -> ColorPalette {
        match self.0.get_bit(Self::COLOR_PALETTE_OFFSET) {
            false => ColorPalette::Is16x16,
            true => match extended_palettes {
                false => ColorPalette::Is256x1,
                true => ColorPalette::Is256x16,
            },
        }
    }

    pub fn get_screen_base_block(&self) -> u16 {
        self.0
            .get_bits(Self::SCREEN_BASE_BLOCK_START, Self::SCREEN_BASE_BLOCK_END)
    }

    pub fn get_ext_palette_slot(&self) -> bool {
        self.0.get_bit(Self::EXT_PALETTE_SLOT_OFFSET)
    }

    pub fn get_screen_size(&self) -> u8 {
        self.0
            .get_bits(Self::SCREEN_SIZE_START, Self::SCREEN_SIZE_END) as u8
    }
}

#[derive(PartialEq)]
pub enum ColorPalette {
    Is16x16,  // 0
    Is256x1,  // 1
    Is256x16, // bit 30
}
