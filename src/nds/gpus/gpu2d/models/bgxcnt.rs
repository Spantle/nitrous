#![allow(dead_code)]

use bitflags::bitflags;

pub struct BGxCNT(u16);

impl BGxCNT {
    const BG_PRIORITY_OFFSET: u16 = 0;
    const CHARACTER_BASE_OFFSET: u16 = 2;
    const MOSAIC_OFFSET: u16 = 6;
    const COLOR_PALETTE_MODE_OFFSET: u16 = 7;
    const SCREEN_BASE_BLOCK_OFFSET: u16 = 8;
    const EXT_PALETTE_SLOT_OFFSET: u16 = 13;
    const SCREEN_SIZE_OFFSET: u16 = 14;

    pub fn value(&self) -> u16 {
        self.0
    }

    fn get_bit(&self, offset: u16) -> bool {
        (self.0 >> offset) & 1 == 1
    }

    // NOTE: this is LENGTH based not END based
    fn get_bits(&self, offset: u16, length: u16) -> u16 {
        (self.0 >> offset) & ((1 << length) - 1)
    }

    fn set_bit(&mut self, offset: u16, value: bool) {
        self.0 = (self.0 & !(1 << offset)) | ((value as u16) << offset);
    }

    // NOTE: this is LENGTH based not END based
    fn set_bits(&mut self, offset: u16, length: u16, value: u16) {
        self.0 = (self.0 & !((1 << length) - 1)) | (value << offset);
    }

    pub fn get_bg_priority(&self) -> u16 {
        self.get_bits(Self::BG_PRIORITY_OFFSET, 2)
    }

    pub fn set_bg_priority(&mut self, bg_priority: u16) {
        self.set_bits(Self::BG_PRIORITY_OFFSET, 2, bg_priority);
    }

    pub fn get_character_base(&self) -> u16 {
        self.get_bits(Self::CHARACTER_BASE_OFFSET, 2)
    }

    pub fn set_character_base(&mut self, character_base: u16) {
        self.set_bits(Self::CHARACTER_BASE_OFFSET, 2, character_base);
    }

    pub fn get_mosaic(&self) -> bool {
        self.get_bit(Self::MOSAIC_OFFSET)
    }

    pub fn set_mosaic(&mut self, mosaic: bool) {
        self.set_bit(Self::MOSAIC_OFFSET, mosaic);
    }

    pub fn get_color_palette_mode(&self) -> bool {
        self.get_bit(Self::COLOR_PALETTE_MODE_OFFSET)
    }

    pub fn set_color_palette_mode(&mut self, color_palette_mode: bool) {
        self.set_bit(Self::COLOR_PALETTE_MODE_OFFSET, color_palette_mode);
    }

    pub fn get_screen_base_block(&self) -> u16 {
        self.get_bits(Self::SCREEN_BASE_BLOCK_OFFSET, 5)
    }

    pub fn set_screen_base_block(&mut self, screen_base_block: u16) {
        self.set_bits(Self::SCREEN_BASE_BLOCK_OFFSET, 5, screen_base_block);
    }

    pub fn get_ext_palette_slot(&self) -> bool {
        self.get_bit(Self::EXT_PALETTE_SLOT_OFFSET)
    }

    pub fn set_ext_palette_slot(&mut self, ext_palette_slot: bool) {
        self.set_bit(Self::EXT_PALETTE_SLOT_OFFSET, ext_palette_slot);
    }

    pub fn get_screen_size(&self) -> ScreenSize {
        ScreenSize::from_bits_truncate(self.get_bits(Self::SCREEN_SIZE_OFFSET, 2))
    }

    pub fn set_screen_size(&mut self, screen_size: ScreenSize) {
        self.set_bits(Self::SCREEN_SIZE_OFFSET, 2, screen_size.bits());
    }
}

bitflags! {
    pub struct ScreenSize: u16 {
        const TXT_256x256   = 0;
        const TXT_512x256   = 1;
        const TXT_256x512   = 2;
        const TXT_512x512   = 3;
        const ROT_128x128   = 0;
        const ROT_256x256   = 1;
        const ROT_512x512   = 2;
        const ROT_1024x1024 = 3;
    }
}
