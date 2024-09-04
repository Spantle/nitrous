use crate::nds::Bits;

pub struct DispStat(u16);

impl Default for DispStat {
    fn default() -> Self {
        Self(0x4)
    }
}

impl From<u16> for DispStat {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl DispStat {
    const VBLANK_FLAG_OFFSET: u16 = 0;
    const HBLANK_FLAG_OFFSET: u16 = 1;
    const VCOUNTER_FLAG_OFFSET: u16 = 2;
    const VBLANK_IRQ_ENABLE_OFFSET: u16 = 3;
    const HBLANK_IRQ_ENABLE_OFFSET: u16 = 4;
    const VCOUNTER_IRQ_ENABLE_OFFSET: u16 = 5;
    const VCOUNT_SETTING_OFFSET: u16 = 7;
    const VCOUNT_SETTING_END: u16 = 15;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn get_vblank_flag(&self) -> bool {
        self.0.get_bit(Self::VBLANK_FLAG_OFFSET)
    }

    pub fn set_vblank_flag(&mut self, value: bool) {
        self.0.set_bit(Self::VBLANK_FLAG_OFFSET, value);
    }

    pub fn get_hblank_flag(&self) -> bool {
        self.0.get_bit(Self::HBLANK_FLAG_OFFSET)
    }

    pub fn set_hblank_flag(&mut self, value: bool) {
        self.0.set_bit(Self::HBLANK_FLAG_OFFSET, value);
    }

    pub fn get_vcounter_flag(&self) -> bool {
        self.0.get_bit(Self::VCOUNTER_FLAG_OFFSET)
    }

    pub fn set_vcounter_flag(&mut self, value: bool) {
        self.0.set_bit(Self::VCOUNTER_FLAG_OFFSET, value);
    }

    pub fn get_vblank_irq_enable(&self) -> bool {
        self.0.get_bit(Self::VBLANK_IRQ_ENABLE_OFFSET)
    }

    pub fn set_vblank_irq_enable(&mut self, value: bool) {
        self.0.set_bit(Self::VBLANK_IRQ_ENABLE_OFFSET, value);
    }

    pub fn get_hblank_irq_enable(&self) -> bool {
        self.0.get_bit(Self::HBLANK_IRQ_ENABLE_OFFSET)
    }

    pub fn set_hblank_irq_enable(&mut self, value: bool) {
        self.0.set_bit(Self::HBLANK_IRQ_ENABLE_OFFSET, value);
    }

    pub fn get_vcounter_irq_enable(&self) -> bool {
        self.0.get_bit(Self::VCOUNTER_IRQ_ENABLE_OFFSET)
    }

    pub fn set_vcounter_irq_enable(&mut self, value: bool) {
        self.0.set_bit(Self::VCOUNTER_IRQ_ENABLE_OFFSET, value);
    }

    pub fn get_vcount_setting(&self) -> u16 {
        self.0
            .get_bits(Self::VCOUNT_SETTING_OFFSET, Self::VCOUNT_SETTING_END)
    }

    pub fn set_vcount_setting(&mut self, value: u16) {
        self.0
            .set_bits(Self::VCOUNT_SETTING_OFFSET, Self::VCOUNT_SETTING_END, value);
    }
}
