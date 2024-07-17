use std::fmt;

use crate::nds::Bits;

#[derive(Debug)]
pub struct Instruction(u32);

impl From<u32> for Instruction {
    fn from(val: u32) -> Self {
        Instruction(val)
    }
}

impl std::fmt::Binary for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032b}", self.0)
    }
}

impl Instruction {
    pub fn get_bit(&self, offset: u32) -> bool {
        self.0.get_bit(offset)
    }

    pub fn get_byte(&self, offset: u32, to: u32) -> u8 {
        self.get_word(offset, to) as u8
    }

    pub fn get_halfword(&self, offset: u32, to: u32) -> u16 {
        self.get_word(offset, to) as u16
    }

    pub fn get_word(&self, offset: u32, to: u32) -> u32 {
        (self.0 >> offset) & ((1 << (to - offset + 1)) - 1)
    }
}
