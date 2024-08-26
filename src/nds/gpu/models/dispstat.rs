#![allow(dead_code)]

#[allow(clippy::upper_case_acronyms)]
pub struct DISPSTAT(u16);

impl Default for DISPSTAT {
    fn default() -> Self {
        Self(0x4)
    }
}

impl DISPSTAT {
    const VBLANK_FLAG_OFFSET: u16 = 0;
    const HBLANK_FLAG_OFFSET: u16 = 1;
    const VCOUNT_FLAG_OFFSET: u16 = 2;
    const VBLANK_IRQ_ENABLE_OFFSET: u16 = 3;
    const HBLANK_IRQ_ENABLE_OFFSET: u16 = 4;
    const VCOUNT_SETTING_OFFSET: u16 = 7;

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

    pub fn get_vblank_flag(&self) -> bool {
        self.get_bit(Self::VBLANK_FLAG_OFFSET)
    }

    pub fn set_vblank_flag(&mut self, value: bool) {
        self.set_bit(Self::VBLANK_FLAG_OFFSET, value);
    }

    pub fn get_hblank_flag(&self) -> bool {
        self.get_bit(Self::HBLANK_FLAG_OFFSET)
    }

    pub fn set_hblank_flag(&mut self, value: bool) {
        self.set_bit(Self::HBLANK_FLAG_OFFSET, value);
    }

    pub fn get_vcount_flag(&self) -> bool {
        self.get_bit(Self::VCOUNT_FLAG_OFFSET)
    }

    pub fn set_vcount_flag(&mut self, value: bool) {
        self.set_bit(Self::VCOUNT_FLAG_OFFSET, value);
    }

    pub fn get_vblank_irq_enable(&self) -> bool {
        self.get_bit(Self::VBLANK_IRQ_ENABLE_OFFSET)
    }

    pub fn set_vblank_irq_enable(&mut self, value: bool) {
        self.set_bit(Self::VBLANK_IRQ_ENABLE_OFFSET, value);
    }

    pub fn get_hblank_irq_enable(&self) -> bool {
        self.get_bit(Self::HBLANK_IRQ_ENABLE_OFFSET)
    }

    pub fn set_hblank_irq_enable(&mut self, value: bool) {
        self.set_bit(Self::HBLANK_IRQ_ENABLE_OFFSET, value);
    }

    pub fn get_vcount_setting(&self) -> u16 {
        self.get_bits(Self::VCOUNT_SETTING_OFFSET, 9)
    }

    pub fn set_vcount_setting(&mut self, value: u16) {
        self.set_bits(Self::VCOUNT_SETTING_OFFSET, 9, value);
    }
}
