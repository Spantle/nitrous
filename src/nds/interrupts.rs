use super::Bits;

#[derive(Default)]
pub struct Interrupts {
    pub me: InterruptMasterEnable, // 0x04000208, IME
    pub e: InterruptFlags,         // 0x04000210, IE
    pub f: InterruptFlags,         // 0x04000214, IF
}

impl Interrupts {
    pub fn is_requesting_interrupt(&self) -> bool {
        self.me.get_disable_all() && (self.e.0 & self.f.0) != 0
    }
}

#[derive(Default)]
pub struct InterruptMasterEnable(u32);

impl From<u32> for InterruptMasterEnable {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl InterruptMasterEnable {
    pub fn value(&self) -> u32 {
        self.0
    }

    // actually inverted, 0 is disable all
    pub fn get_disable_all(&self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_disable_all(&mut self, value: bool) {
        self.0.set_bit(0, value);
    }
}

#[derive(Default)]
pub struct InterruptFlags(u32);

impl From<u32> for InterruptFlags {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl InterruptFlags {
    const LCD_VBLANK_OFFSET: u32 = 0;
    const LCD_HBLANK_OFFSET: u32 = 1;
    const LCD_VCOUNTER_MATCH_OFFSET: u32 = 2;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn get_lcd_vblank(&self) -> bool {
        self.0.get_bit(Self::LCD_VBLANK_OFFSET)
    }

    pub fn set_lcd_vblank(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_VBLANK_OFFSET, value);
    }

    pub fn get_lcd_hblank(&self) -> bool {
        self.0.get_bit(Self::LCD_HBLANK_OFFSET)
    }

    pub fn set_lcd_hblank(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_HBLANK_OFFSET, value);
    }

    pub fn get_lcd_vcounter_match(&self) -> bool {
        self.0.get_bit(Self::LCD_VCOUNTER_MATCH_OFFSET)
    }

    pub fn set_lcd_vcounter_match(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_VCOUNTER_MATCH_OFFSET, value);
    }
}
