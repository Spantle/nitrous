use std::fmt;

use crate::nds::Bits;

pub struct Instruction(u16);

impl From<u16> for Instruction {
    fn from(val: u16) -> Self {
        Instruction(val)
    }
}

impl std::fmt::Binary for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl Instruction {
    #[inline(always)]
    pub fn get_bit(&self, offset: u16) -> bool {
        self.0.get_bit(offset)
    }

    #[inline(always)]
    pub fn get_byte(&self, offset: u32, to: u32) -> u8 {
        self.get_word(offset, to) as u8
    }

    #[inline(always)]
    pub fn get_halfword(&self, offset: u32, to: u32) -> u16 {
        self.get_word(offset, to) as u16
    }

    #[inline(always)]
    pub fn get_word(&self, offset: u32, to: u32) -> u32 {
        ((self.0 >> offset) & ((1 << (to - offset + 1)) - 1)) as u32
    }
}
