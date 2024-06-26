#![allow(dead_code)]

use bitflags::bitflags;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct DISPCNT(u32);

impl From<u32> for DISPCNT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl DISPCNT {
    const BG_MODE_OFFSET: u32 = 0;
    const BG0_3D_OFFSET: u32 = 3;
    const TILE_OBJ_MAPPING_OFFSET: u32 = 4;
    const BITMAP_OBJ_2D_DIMENSION_OFFSET: u32 = 5;
    const BITMAP_OBJ_MAPPING_OFFSET: u32 = 6;
    const FORCE_BLANK_OFFSET: u32 = 7;
    const DISPLAY_BG0_OFFSET: u32 = 8;
    const DISPLAY_BG1_OFFSET: u32 = 9;
    const DISPLAY_BG2_OFFSET: u32 = 10;
    const DISPLAY_BG3_OFFSET: u32 = 11;
    const DISPLAY_OBJ_OFFSET: u32 = 12;
    const DISPLAY_WIN0_OFFSET: u32 = 13;
    const DISPLAY_WIN1_OFFSET: u32 = 14;
    const DISPLAY_OBJ_WIN_OFFSET: u32 = 15;
    const DISPLAY_MODE_OFFSET: u32 = 16;
    const VRAM_BLOCK_OFFSET: u32 = 18;
    const TILE_OBJ_1D_BOUNDARY_OFFSET: u32 = 20;
    const BITMAP_OBJ_1D_BOUNDARY_OFFSET: u32 = 22;
    const OBJ_PROCESSING_DURING_HBLANK_OFFSET: u32 = 23;
    const CHARACTER_BASE_OFFSET: u32 = 24;
    const SCREEN_BASE_OFFSET: u32 = 27;
    const BG_EXTENDED_PALETTES_OFFSET: u32 = 30;
    const OBJ_EXTENDED_PALETTES_OFFSET: u32 = 31;

    pub fn value(&self) -> u32 {
        self.0
    }

    fn get_bit(&self, offset: u32) -> bool {
        (self.0 >> offset) & 1 == 1
    }

    // NOTE: this is LENGTH based not END based
    fn get_bits(&self, offset: u32, length: u32) -> u32 {
        (self.0 >> offset) & ((1 << length) - 1)
    }

    fn set_bit(&mut self, offset: u32, value: bool) {
        self.0 = (self.0 & !(1 << offset)) | ((value as u32) << offset);
    }

    // NOTE: this is LENGTH based not END based
    fn set_bits(&mut self, offset: u32, length: u32, value: u32) {
        self.0 = (self.0 & !((1 << length) - 1)) | (value << offset);
    }

    pub fn get_bg_mode(&self) -> u32 {
        self.get_bits(Self::BG_MODE_OFFSET, 3)
    }

    pub fn set_bg_mode(&mut self, bg_mode: u32) {
        self.set_bits(Self::BG_MODE_OFFSET, 3, bg_mode);
    }

    pub fn get_bg0_3d(&self) -> bool {
        self.get_bit(Self::BG0_3D_OFFSET)
    }

    pub fn set_bg0_3d(&mut self, bg0_3d: bool) {
        self.set_bit(Self::BG0_3D_OFFSET, bg0_3d);
    }

    pub fn get_tile_obj_mapping(&self) -> bool {
        self.get_bit(Self::TILE_OBJ_MAPPING_OFFSET)
    }

    pub fn set_tile_obj_mapping(&mut self, tile_obj_mapping: bool) {
        self.set_bit(Self::TILE_OBJ_MAPPING_OFFSET, tile_obj_mapping);
    }

    pub fn get_bitmap_obj_2d_dimension(&self) -> bool {
        self.get_bit(Self::BITMAP_OBJ_2D_DIMENSION_OFFSET)
    }

    pub fn set_bitmap_obj_2d_dimension(&mut self, bitmap_obj_2d_dimension: bool) {
        self.set_bit(
            Self::BITMAP_OBJ_2D_DIMENSION_OFFSET,
            bitmap_obj_2d_dimension,
        );
    }

    pub fn get_bitmap_obj_mapping(&self) -> bool {
        self.get_bit(Self::BITMAP_OBJ_MAPPING_OFFSET)
    }

    pub fn set_bitmap_obj_mapping(&mut self, bitmap_obj_mapping: bool) {
        self.set_bit(Self::BITMAP_OBJ_MAPPING_OFFSET, bitmap_obj_mapping);
    }

    pub fn get_force_blank(&self) -> bool {
        self.get_bit(Self::FORCE_BLANK_OFFSET)
    }

    pub fn set_force_blank(&mut self, force_blank: bool) {
        self.set_bit(Self::FORCE_BLANK_OFFSET, force_blank);
    }

    pub fn get_display_bg0(&self) -> bool {
        self.get_bit(Self::DISPLAY_BG0_OFFSET)
    }

    pub fn set_display_bg0(&mut self, display_bg0: bool) {
        self.set_bit(Self::DISPLAY_BG0_OFFSET, display_bg0);
    }

    pub fn get_display_bg1(&self) -> bool {
        self.get_bit(Self::DISPLAY_BG1_OFFSET)
    }

    pub fn set_display_bg1(&mut self, display_bg1: bool) {
        self.set_bit(Self::DISPLAY_BG1_OFFSET, display_bg1);
    }

    pub fn get_display_bg2(&self) -> bool {
        self.get_bit(Self::DISPLAY_BG2_OFFSET)
    }

    pub fn set_display_bg2(&mut self, display_bg2: bool) {
        self.set_bit(Self::DISPLAY_BG2_OFFSET, display_bg2);
    }

    pub fn get_display_bg3(&self) -> bool {
        self.get_bit(Self::DISPLAY_BG3_OFFSET)
    }

    pub fn set_display_bg3(&mut self, display_bg3: bool) {
        self.set_bit(Self::DISPLAY_BG3_OFFSET, display_bg3);
    }

    pub fn get_display_obj(&self) -> bool {
        self.get_bit(Self::DISPLAY_OBJ_OFFSET)
    }

    pub fn set_display_obj(&mut self, display_obj: bool) {
        self.set_bit(Self::DISPLAY_OBJ_OFFSET, display_obj);
    }

    pub fn get_display_win0(&self) -> bool {
        self.get_bit(Self::DISPLAY_WIN0_OFFSET)
    }

    pub fn set_display_win0(&mut self, display_win0: bool) {
        self.set_bit(Self::DISPLAY_WIN0_OFFSET, display_win0);
    }

    pub fn get_display_win1(&self) -> bool {
        self.get_bit(Self::DISPLAY_WIN1_OFFSET)
    }

    pub fn set_display_win1(&mut self, display_win1: bool) {
        self.set_bit(Self::DISPLAY_WIN1_OFFSET, display_win1);
    }

    pub fn get_display_obj_win(&self) -> bool {
        self.get_bit(Self::DISPLAY_OBJ_WIN_OFFSET)
    }

    pub fn set_display_obj_win(&mut self, display_obj_win: bool) {
        self.set_bit(Self::DISPLAY_OBJ_WIN_OFFSET, display_obj_win);
    }

    pub fn get_display_mode(&self) -> DisplayMode {
        DisplayMode::from_bits_truncate(self.get_bits(Self::DISPLAY_MODE_OFFSET, 2))
    }

    pub fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.set_bits(Self::DISPLAY_MODE_OFFSET, 2, display_mode.bits());
    }

    pub fn get_vram_block(&self) -> u32 {
        self.get_bits(Self::VRAM_BLOCK_OFFSET, 2)
    }

    pub fn set_vram_block(&mut self, vram_block: u32) {
        self.set_bits(Self::VRAM_BLOCK_OFFSET, 2, vram_block);
    }

    pub fn get_tile_obj_1d_boundary(&self) -> bool {
        self.get_bit(Self::TILE_OBJ_1D_BOUNDARY_OFFSET)
    }

    pub fn set_tile_obj_1d_boundary(&mut self, tile_obj_1d_boundary: bool) {
        self.set_bit(Self::TILE_OBJ_1D_BOUNDARY_OFFSET, tile_obj_1d_boundary);
    }

    pub fn get_bitmap_obj_1d_boundary(&self) -> bool {
        self.get_bit(Self::BITMAP_OBJ_1D_BOUNDARY_OFFSET)
    }

    pub fn set_bitmap_obj_1d_boundary(&mut self, bitmap_obj_1d_boundary: bool) {
        self.set_bit(Self::BITMAP_OBJ_1D_BOUNDARY_OFFSET, bitmap_obj_1d_boundary);
    }

    pub fn get_obj_processing_during_hblank(&self) -> bool {
        self.get_bit(Self::OBJ_PROCESSING_DURING_HBLANK_OFFSET)
    }

    pub fn set_obj_processing_during_hblank(&mut self, obj_processing_during_hblank: bool) {
        self.set_bit(
            Self::OBJ_PROCESSING_DURING_HBLANK_OFFSET,
            obj_processing_during_hblank,
        );
    }

    pub fn get_character_base(&self) -> u32 {
        self.get_bits(Self::CHARACTER_BASE_OFFSET, 3)
    }

    pub fn set_character_base(&mut self, character_base: u32) {
        self.set_bits(Self::CHARACTER_BASE_OFFSET, 3, character_base);
    }

    pub fn get_screen_base(&self) -> u32 {
        self.get_bits(Self::SCREEN_BASE_OFFSET, 3)
    }

    pub fn set_screen_base(&mut self, screen_base: u32) {
        self.set_bits(Self::SCREEN_BASE_OFFSET, 3, screen_base);
    }

    pub fn get_bg_extended_palettes(&self) -> bool {
        self.get_bit(Self::BG_EXTENDED_PALETTES_OFFSET)
    }

    pub fn set_bg_extended_palettes(&mut self, bg_extended_palettes: bool) {
        self.set_bit(Self::BG_EXTENDED_PALETTES_OFFSET, bg_extended_palettes);
    }

    pub fn get_obj_extended_palettes(&self) -> bool {
        self.get_bit(Self::OBJ_EXTENDED_PALETTES_OFFSET)
    }

    pub fn set_obj_extended_palettes(&mut self, obj_extended_palettes: bool) {
        self.set_bit(Self::OBJ_EXTENDED_PALETTES_OFFSET, obj_extended_palettes);
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
